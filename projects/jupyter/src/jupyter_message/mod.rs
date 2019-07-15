// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use crate::{
    client::ExecuteContext,
    connection::{Connection, HmacSha256},
    errors::JupyterError,
    JupyterResult,
};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use generic_array::GenericArray;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_slice, from_str, to_string, to_vec, Map, Value};
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    str::FromStr,
    {self},
};
use tokio::task::JoinError;
use uuid::Uuid;
use zeromq::{SocketRecv, SocketSend, ZmqMessage};
mod der;
mod kernel_info;
mod message_type;
mod ser;
pub use self::{kernel_info::KernelInfo, message_type::JupyterMessageType};

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
            use hmac::Mac;
            if let Err(error) = mac.verify(GenericArray::from_slice(&hex::decode(&hmac)?)) {
                panic!("{}", error);
            }
        }

        Ok(raw_message)
    }

    async fn send<S: SocketSend>(self, connection: &mut Connection<S>) -> JupyterResult<()> {
        use hmac::Mac;
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
        use hmac::Mac;
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
    content: JupiterContent,
}

#[derive(Clone)]
pub enum JupiterContent {
    ExecutionState(Box<ExecutionState>),
    KernelInfo(Box<KernelInfo>),
    Custom(Box<Value>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionState {
    execution_state: String,
}

impl From<ExecutionState> for JupiterContent {
    fn from(value: ExecutionState) -> Self {
        JupiterContent::ExecutionState(Box::new(value))
    }
}

impl ExecutionState {
    pub fn new<S: ToString>(state: S) -> Self {
        Self { execution_state: state.to_string() }
    }
}

impl Debug for JupiterContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JupiterContent::KernelInfo(v) => Debug::fmt(v, f),
            JupiterContent::Custom(v) => Debug::fmt(v, f),
            JupiterContent::ExecutionState(v) => Debug::fmt(v, f),
        }
    }
}

impl Default for JupiterContent {
    fn default() -> Self {
        JupiterContent::Custom(Box::new(Value::Null))
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
    pub fn kind(&self) -> &JupyterMessageType {
        &self.header.msg_type
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
            content: from_slice(&raw_message.jparts[3])?,
        })
    }

    pub(crate) fn message_type(&self) -> &str {
        self.header.msg_type.as_ref()
    }

    pub(crate) fn code(&self) -> &str {
        match &self.content {
            JupiterContent::Custom(v) => v["code"].as_str().unwrap_or(""),
            _ => "",
        }
    }

    pub(crate) fn cursor_pos(&self) -> usize {
        match &self.content {
            JupiterContent::Custom(v) => v["cursor_pos"].as_u64().unwrap_or(0) as usize,
            _ => 0,
        }
        // self.content["cursor_pos"].as_usize().unwrap_or_default()
    }

    pub(crate) fn target_name(&self) -> &str {
        match &self.content {
            JupiterContent::Custom(v) => v["target_name"].as_str().unwrap_or(""),
            _ => "",
        }
    }

    pub(crate) fn data(&self) -> &Value {
        match &self.content {
            JupiterContent::Custom(v) => &v["data"],
            _ => &Value::Null,
        }
    }

    pub(crate) fn comm_id(&self) -> &str {
        match &self.content {
            JupiterContent::Custom(v) => v["comm_id"].as_str().unwrap_or(""),
            _ => "",
        }
    }

    pub(crate) fn new(msg_type: &str) -> JupyterMessage {
        JupyterMessage {
            zmq_identities: Vec::new(),
            header: JupyterMessageHeader {
                username: "kernel".to_string(),
                msg_type: JupyterMessageType::from_str(msg_type).unwrap_or_default(),
                date: Utc::now(),
                msg_id: Uuid::new_v4(),
                session: Uuid::nil(),
                version: "".to_string(),
            },
            parent_header: JupyterMessageHeader::default(),
            metadata: Value::Null,
            content: JupiterContent::default(),
        }
    }

    // Creates a new child message of this message. ZMQ identities are not transferred.
    pub fn create_message(&self, kind: JupyterMessageType) -> JupyterMessage {
        JupyterMessage {
            zmq_identities: Vec::new(),
            header: JupyterMessageHeader {
                date: Utc::now(),
                msg_id: Uuid::new_v4(),
                msg_type: kind,
                session: self.header.session.clone(),
                username: "kernel".to_string(),
                version: self.header.version.clone(),
            },
            parent_header: self.header.clone(),
            metadata: Value::Null,
            content: JupiterContent::default(),
        }
    }

    // Creates a reply to this message. This is a child with the message type determined
    // automatically by replacing "request" with "reply". ZMQ identities are transferred.
    pub fn as_reply(&self) -> JupyterMessage {
        let mut reply = self.create_message(self.header.msg_type.as_reply());
        reply.zmq_identities = self.zmq_identities.clone();
        reply
    }

    #[must_use = "Need to send this message for it to have any effect"]
    pub(crate) fn comm_close_message(&self) -> JupyterMessage {
        todo!()
    }

    pub fn get_content(&self) -> &JupiterContent {
        &self.content
    }

    pub fn with_content<T>(mut self, content: T) -> JupyterMessage
    where
        T: Into<JupiterContent>,
    {
        self.content = content.into();
        self
    }

    pub(crate) fn with_message_type(mut self, msg_type: &str) -> JupyterMessage {
        self.header.msg_type = JupyterMessageType::from_str(msg_type).unwrap_or_default();
        self
    }

    pub(crate) fn without_parent_header(mut self) -> JupyterMessage {
        self.parent_header = JupyterMessageHeader::default();
        self
    }

    pub(crate) async fn send<S: SocketSend>(&self, connection: &mut Connection<S>) -> JupyterResult<()> {
        // If performance is a concern, we can probably avoid the clone and to_vec calls with a bit
        // of refactoring.
        let raw_message = RawMessage {
            zmq_identities: self.zmq_identities.clone(),
            jparts: vec![
                to_vec(&self.header)?.into(),
                to_vec(&self.parent_header)?.into(),
                to_vec(&self.metadata)?.into(),
                to_vec(&self.content)?.into(),
            ],
        };
        raw_message.send(connection).await
    }
}
