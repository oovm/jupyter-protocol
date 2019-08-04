#![allow(deprecated)]
use crate::JupiterContent;
use serde::{
    ser::{SerializeMap, SerializeStruct},
    Serialize, Serializer,
};

#[derive(Clone, Debug)]
pub struct ExecutionReply {
    success: bool,
    execution_count: u32,
    payload: Vec<ExecutionPayload>,
}

#[deprecated]
#[derive(Clone, Debug)]
pub enum ExecutionPayload {
    Page {
        mime: String,
        /// line offset to start from
        start: i32,
    },
    NextInput {
        text: String,
        replace: bool,
    },
}

impl From<ExecutionReply> for JupiterContent {
    fn from(value: ExecutionReply) -> Self {
        JupiterContent::ExecutionReply(Box::new(value))
    }
}

impl Serialize for ExecutionReply {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_struct("ExecutionReply", 5)?;
        match self.success {
            true => map.serialize_field("status", "ok")?,
            false => map.serialize_field("status", "error")?,
        }
        map.serialize_field("execution_count", &self.execution_count)?;
        if !self.payload.is_empty() {
            map.serialize_field("payload", &self.payload)?;
        }
        map.end()
    }
}

impl Serialize for ExecutionPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ExecutionPayload::Page { mime: data, start } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("source", "page")?;
                map.serialize_entry("data", data)?;
                map.serialize_entry("start", start)?;
                map.end()
            }
            ExecutionPayload::NextInput { text, replace } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("source", "set_next_input")?;
                map.serialize_entry("text", text)?;
                map.serialize_entry("replace", replace)?;
                map.end()
            }
        }
    }
}

impl ExecutionReply {
    pub fn new(success: bool, execution_count: u32) -> Self {
        Self { success, execution_count, payload: vec![] }
    }
}
