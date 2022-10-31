pub use self::{common_info::CommonInfoRequest, debug_info::DebugRequest, execute::ExecutionResult};
use crate::{
    connection::{Connection, HmacSha256},
    errors::JupyterError,
    JupyterResult,
};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use generic_array::GenericArray;
use hmac::Mac;
use serde::{de::DeserializeOwned, ser::SerializeMap, Deserialize, Serialize, Serializer};
use serde_json::{from_slice, from_value, to_value, to_vec, Value};
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    str::FromStr,
    {self},
};
use uuid::Uuid;
use zeromq::{SocketRecv, SocketSend};
mod common_info;
mod debug_info;
mod der;
mod execute;
mod interrupt;
mod kernel_info;
mod message_type;
mod ser;
mod shutdown;
pub use self::{execute::ExecutionRequest, kernel_info::KernelInfoReply, message_type::JupyterMessageType};

struct RawMessage {
    zmq_identities: Vec<Bytes>,
    jparts: Vec<Bytes>,
}

impl RawMessage {
    pub(crate) async fn read<S: SocketRecv>(connection: &mut Connection<S>) -> JupyterResult<RawMessage> {
        Self::from_multipart(connection.socket.recv().await?, connection)
    }

    pub(crate) fn from_multipart<S>(multipart: zeromq::ZmqMessage, connection: &Connection<S>) -> JupyterResult<RawMessage> {
        let delimiter_index =
            multipart.iter().position(|part| &part[..] == DELIMITER).ok_or_else(|| panic!("Missing delimeter"))?;
        let mut parts = multipart.into_vec();
        let jparts: Vec<_> = parts.drain(delimiter_index + 2..).collect();
        let hmac = parts.pop().unwrap();
        // Remove delimiter, so that what's left is just the identities.
        parts.pop();
        let zmq_identities = parts;

        let raw_message = RawMessage { zmq_identities, jparts };

        if let Some(mac_template) = &connection.mac {
            let mut mac = mac_template.clone();
            raw_message.digest(&mut mac);

            if let Err(error) = mac.verify(GenericArray::from_slice(&hex::decode(&hmac)?)) {
                panic!("{}", error);
            }
        }

        Ok(raw_message)
    }

    async fn send<S: SocketSend>(self, connection: &mut Connection<S>) -> JupyterResult<()> {
        let hmac = if let Some(mac_template) = &connection.mac {
            let mut mac = mac_template.clone();
            self.digest(&mut mac);
            hex::encode(mac.finalize().into_bytes().as_slice())
        }
        else {
            String::new()
        };
        let mut parts: Vec<bytes::Bytes> = Vec::new();
        for part in &self.zmq_identities {
            parts.push(part.to_vec().into());
        }
        parts.push(DELIMITER.into());
        parts.push(hmac.as_bytes().to_vec().into());
        for part in &self.jparts {
            parts.push(part.to_vec().into());
        }
        // ZmqMessage::try_from only fails if parts is empty, which it never
        // will be here.
        let message = zeromq::ZmqMessage::try_from(parts).unwrap();
        connection.socket.send(message).await?;
        Ok(())
    }

    fn digest(&self, mac: &mut HmacSha256) {
        for part in &self.jparts {
            mac.update(part);
        }
    }
}

#[derive(Clone, Debug)]
pub struct JupyterMessage {
    zmq_identities: Vec<Bytes>,
    header: JupyterMessageHeader,
    parent_header: JupyterMessageHeader,
    metadata: Value,
    content: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionState {
    execution_state: String,
}

impl ExecutionState {
    pub fn new<S: ToString>(state: S) -> Self {
        Self { execution_state: state.to_string() }
    }
}

#[derive(Clone, Debug)]
pub struct JupyterMessageHeader {
    pub username: String,
    pub msg_type: JupyterMessageType,
    pub date: DateTime<Utc>,
    pub session: Uuid,
    pub msg_id: Uuid,
    pub version: String,
}

impl Default for JupyterMessageHeader {
    fn default() -> Self {
        Self {
            date: Default::default(),
            msg_id: Default::default(),
            msg_type: Default::default(),
            session: Default::default(),
            username: "".to_string(),
            version: "".to_string(),
        }
    }
}

const DELIMITER: &[u8] = b"<IDS|MSG>";

impl JupyterMessage {
    pub(crate) async fn read<S: SocketRecv>(connection: &mut Connection<S>) -> JupyterResult<JupyterMessage> {
        Self::from_raw_message(RawMessage::read(connection).await?)
    }
    /// Get the message type.
    pub fn kind(&self) -> &JupyterMessageType {
        &self.header.msg_type
    }
    /// Change weakly typed content into strongly typed content.
    pub fn recast<T: DeserializeOwned>(&self) -> JupyterResult<T> {
        match from_value(self.content.clone()) {
            Ok(v) => Ok(v),
            Err(e) => Err(JupyterError::any(format!("Expected {} but got {}", std::any::type_name::<T>(), e))),
        }
    }
    fn from_raw_message(raw_message: RawMessage) -> JupyterResult<JupyterMessage> {
        fn message_to_json(message: &[u8]) -> JupyterResult<Value> {
            let out = Value::from_str(std::str::from_utf8(message).unwrap_or("")).unwrap();
            Ok(out)
        }

        if raw_message.jparts.len() < 4 {
            panic!("Insufficient message parts {}", raw_message.jparts.len());
        }

        Ok(JupyterMessage {
            zmq_identities: raw_message.zmq_identities,
            header: from_slice(&raw_message.jparts[0])?,
            parent_header: from_slice(&raw_message.jparts[1])?,
            metadata: message_to_json(&raw_message.jparts[2])?,
            content: from_slice::<Value>(&raw_message.jparts[3])?,
        })
    }

    // Creates a new child message of this message. ZMQ identities are not transferred.
    pub fn create_message(&self, kind: JupyterMessageType) -> JupyterMessage {
        JupyterMessage {
            zmq_identities: Vec::new(),
            header: JupyterMessageHeader {
                username: "kernel".to_string(),
                session: self.header.session.clone(),
                version: self.header.version.clone(),
                msg_id: Uuid::new_v4(),
                msg_type: kind,
                date: Utc::now(),
            },
            parent_header: self.header.clone(),
            metadata: Value::Null,
            content: Value::Null,
        }
    }

    // Creates a reply to this message. This is a child with the message type determined
    // automatically by replacing "request" with "reply". ZMQ identities are transferred.
    pub fn as_reply(&self) -> JupyterMessage {
        let mut reply = self.create_message(self.header.msg_type.as_reply());
        reply.zmq_identities = self.zmq_identities.clone();
        reply
    }

    pub fn with_content<T: Serialize>(mut self, content: T) -> JupyterResult<JupyterMessage> {
        self.content = to_value(content)?;
        Ok(self)
    }

    pub fn with_message_type(mut self, msg_type: JupyterMessageType) -> JupyterMessage {
        self.header.msg_type = msg_type;
        self
    }

    pub fn drop_parent_header(&mut self) {
        self.parent_header = JupyterMessageHeader::default();
    }

    pub(crate) async fn send<S: SocketSend>(&self, connection: &mut Connection<S>) -> JupyterResult<()> {
        // If performance is a concern, we can probably avoid the clone and to_vec calls with a bit of refactoring.
        let raw = RawMessage {
            zmq_identities: self.zmq_identities.clone(),
            jparts: vec![
                to_vec(&self.header)?.into(),
                to_vec(&self.parent_header)?.into(),
                to_vec(&self.metadata)?.into(),
                to_vec(&self.content)?.into(),
            ],
        };
        raw.send(connection).await
    }
}
