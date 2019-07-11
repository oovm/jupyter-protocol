use super::*;
use crate::errors::JupyterError;
use chrono::{DateTime, ParseResult, TimeZone};
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserializer,
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
        let mut msg_type = JupyterMessageType::default();
        while let Some(key) = map.next_key()? {
            match key {
                "date" => {
                    let rfc3339 = map.next_value::<String>()?;
                    if let Ok(o) = DateTime::parse_from_rfc3339(&rfc3339) {
                        date = o.with_timezone(&Utc)
                    }
                }
                "msg_id" => {
                    let v4 = map.next_value::<String>()?;
                    let (head, _) = v4.split_at(36);
                    if let Ok(o) = Uuid::parse_str(head) {
                        msg_id = o;
                    }
                }
                "msg_type" => msg_type = map.next_value()?,
                "session" => self.session = map.next_value()?,
                "username" => self.username = map.next_value()?,
                "version" => self.version = map.next_value()?,
                _ => {
                    print!("Unknown key {}", key)
                }
            }
        }
        Ok(JupyterMessageHeader {
            date,
            msg_id,
            msg_type,
            session: self.session,
            username: self.username,
            version: self.version,
        })
    }
}

impl Error for JupyterError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        todo!()
        // JupyterError::custom(msg.to_string())
    }
}