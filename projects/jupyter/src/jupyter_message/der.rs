use super::*;
use crate::errors::JupyterError;
use chrono::DateTime;
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserializer,
    __private::de::{Content, ContentRefDeserializer},
};
use std::fmt::Display;

pub struct JupyterMessageHeaderVisitor {
    session: String,
    username: String,
    version: String,
}

impl<'de> Deserialize<'de> for JupyterMessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let out = JupyterMessageType::from_str(s.as_str());
        unsafe { Ok(out.unwrap_unchecked()) }
    }
}

impl<'de> Deserialize<'de> for JupyterMessageHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(JupyterMessageHeaderVisitor {
            session: "".to_string(),
            username: "".to_string(),
            version: "".to_string(),
        })
    }
}

impl<'de> Visitor<'de> for JupyterMessageHeaderVisitor {
    type Value = JupyterMessageHeader;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("struct JupyterMessageHeader")
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut date = Utc::now();
        let mut msg_id = Uuid::nil();
        let mut session = Uuid::nil();
        let mut msg_type = JupyterMessageType::default();
        while let Some(key) = map.next_key()? {
            match key {
                "date" => {
                    let rfc3339 = map.next_value::<String>()?;
                    if let Ok(o) = DateTime::parse_from_rfc3339(&rfc3339) {
                        date = o.with_timezone(&Utc)
                    }
                }
                "msg_type" => msg_type = map.next_value()?,
                "msg_id" => {
                    let v4 = map.next_value::<String>()?;
                    let head = v4.split('_').next().unwrap_or("");
                    if let Ok(o) = Uuid::parse_str(head) {
                        msg_id = o;
                    }
                }
                "session" => {
                    let v4 = map.next_value::<String>()?;
                    let head = v4.split('_').next().unwrap_or("");
                    if let Ok(o) = Uuid::parse_str(head) {
                        session = o;
                    }
                }
                "username" => self.username = map.next_value()?,
                "version" => self.version = map.next_value()?,
                _ => {
                    print!("Unknown key {}", key)
                }
            }
        }
        Ok(JupyterMessageHeader { date, msg_id, msg_type, session, username: self.username, version: self.version })
    }
}

impl<'de> Deserialize<'de> for JupiterContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let content = Content::deserialize(deserializer)?;
        if let Ok(o) = KernelInfo::deserialize(ContentRefDeserializer::<D::Error>::new(&content)) {
            return Ok(JupiterContent::KernelInfo(Box::new(o)));
        }
        if let Ok(o) = ExecutionRequest::deserialize(ContentRefDeserializer::<D::Error>::new(&content)) {
            return Ok(JupiterContent::ExecutionRequest(Box::new(o)));
        }
        if let Ok(o) = Value::deserialize(ContentRefDeserializer::<D::Error>::new(&content)) {
            return Ok(JupiterContent::Custom(Box::new(o)));
        }
        Ok(JupiterContent::default())
    }
}

impl Error for JupyterError {
    fn custom<T>(der: T) -> Self
    where
        T: Display,
    {
        JupyterError::any(der.to_string())
    }
}
