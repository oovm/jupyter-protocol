use super::*;
use crate::errors::JupyterError;
use chrono::{DateTime, ParseResult, TimeZone};
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserializer,
};
use std::fmt::Display;

pub struct JupyterMessageHeaderVisitor {
    msg_id: Option<Uuid>,
    msg_type: String,
    session: String,
    username: String,
    version: String,
}

impl<'de> Deserialize<'de> for JupyterMessageHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(JupyterMessageHeaderVisitor {
            msg_id: None,
            msg_type: "".to_string(),
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
        while let Some(key) = map.next_key()? {
            match key {
                "date" => {
                    let rfc3339 = map.next_value::<String>()?;
                    if let Ok(o) = DateTime::parse_from_rfc3339(&rfc3339) {
                        date = o.with_timezone(&Utc)
                    }
                }
                "msg_id" => {
                    let uuid = map.next_value::<String>()?;
                    let (head, _) = uuid.split_at(36);
                    match Uuid::parse_str(head) {
                        Ok(o) => {
                            self.msg_id = Some(o);
                        }
                        Err(_) => {
                            return Err(Error::custom(format!("Invalid uuid format {}", uuid)));
                        }
                    }
                }
                "msg_type" => self.msg_type = map.next_value()?,
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
            msg_id: self.msg_id.ok_or(Error::missing_field("msg_id"))?,
            msg_type: self.msg_type,
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
