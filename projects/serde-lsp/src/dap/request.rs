use super::*;
use serde::{
    de::{DeserializeOwned, MapAccess, Visitor},
    Deserializer,
};
use serde_json::{Error, Value};
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct Request {
    pub sequence: usize,
    pub command: String,
    pub arguments: Value,
}

impl Request {
    pub fn recast<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_value(self.arguments.clone())
    }
}

#[derive(Default)]
pub struct RequestVisitor {
    pub sequence: usize,
    pub command: String,
    pub arguments: Value,
}

impl<'de> Deserialize<'de> for Request {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RequestVisitor::default())
    }
}

impl<'i, 'de> Visitor<'de> for RequestVisitor {
    type Value = Request;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a request map")
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "command" => self.command = map.next_value()?,
                "seq" => self.sequence = map.next_value()?,
                "type" => {
                    let _type = map.next_value::<String>()?;
                    debug_assert_eq!(_type, "request")
                }
                "arguments" => self.arguments = map.next_value()?,
                _ => {
                    eprintln!("Unknown key in request: {}", key)
                }
            }
        }
        Ok(Request { sequence: self.sequence, command: self.command, arguments: self.arguments })
    }
}
