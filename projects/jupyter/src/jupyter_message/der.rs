use super::*;
use crate::errors::JupyterError;
use chrono::{DateTime, ParseResult, TimeZone};
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserializer,
};
use std::fmt::Display;

pub struct JupyterMessageHeaderVisitor {
    date: Option<DateTime<Utc>>,
    msg_id: Option<Uuid>,
    msg_type: String,
    session: String,
    username: String,
    version: String,
}

impl TryFrom<JupyterMessageHeaderVisitor> for JupyterMessageHeader {
    type Error = JupyterError;

    fn try_from(value: JupyterMessageHeaderVisitor) -> Result<Self, Self::Error> {
        Ok(JupyterMessageHeader {
            date: value.date.ok_or(JupyterError::missing_field("date"))?,
            msg_id: value.msg_id.ok_or(JupyterError::missing_field("msg_id"))?,
            msg_type: value.msg_type,
            session: value.session,
            username: value.username,
            version: value.version,
        })
    }
}

impl<'de> Deserialize<'de> for JupyterMessageHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "JupyterMessageHeader",
            &["data", "msg_id", "msg_type", "session", "username", "version"],
            JupyterMessageHeaderVisitor {
                date: None,
                msg_id: None,
                msg_type: "".to_string(),
                session: "".to_string(),
                username: "".to_string(),
                version: "".to_string(),
            },
        )
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
        while let Some(key) = map.next_key()? {
            match key {
                "date" => {
                    let rfc3339 = map.next_value::<String>()?;
                    match DateTime::parse_from_rfc3339(&rfc3339) {
                        Ok(o) => {
                            self.date = Some(o.with_timezone(&Utc));
                        }
                        Err(_) => {
                            return Err(Error::custom(format!("Invalid date format {}", rfc3339)));
                        }
                    }
                }
                "msg_id" => {
                    self.msg_id = Some(map.next_value()?);
                }
                "msg_type" => {
                    self.msg_type = map.next_value()?;
                }
                "session" => {
                    self.session = map.next_value()?;
                }
                "username" => {
                    self.username = map.next_value()?;
                }
                "version" => {
                    self.version = map.next_value()?;
                }
                _ => {
                    print!("Unknown key {}", key)
                }
            }
        }
        Ok(JupyterMessageHeader {
            date: self.date.ok_or(Error::missing_field("date"))?,
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
