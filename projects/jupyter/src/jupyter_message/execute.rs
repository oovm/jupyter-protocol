use super::*;
use serde::{
    ser::{SerializeMap, SerializeStruct},
    Serializer,
};
use serde_json::Map;

pub struct ExecutionGroup {
    pub message: JupyterMessage,
    pub request: ExecutionRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionResult {
    execution_count: u32,
    data: Map<String, Value>,
    metadata: Map<String, Value>,
    transient: Map<String, Value>,
}

impl Default for ExecutionResult {
    fn default() -> Self {
        Self { execution_count: 0, data: Map::new(), metadata: Map::new(), transient: Map::new() }
    }
}

impl From<ExecutionResult> for JupiterContent {
    fn from(value: ExecutionResult) -> Self {
        JupiterContent::ExecutionResult(Box::new(value))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub code: String,
    pub silent: bool,
    pub store_history: bool,
    pub allow_stdin: bool,
    pub stop_on_error: bool,
    pub user_expressions: Value,
    #[serde(default)]
    pub execution_count: u32,
}

/// {
//   "source": "page",
//   # mime-bundle of data to display in the pager.
//   # Must include text/plain.
//   "data": mimebundle,
//   # line offset to start from
//   "start": int,
// }
#[derive(Clone, Debug)]
pub struct ExecutionReply {
    success: bool,
    execution_count: u32,
    payload: Vec<ExecutionPayload>,
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

impl Serialize for ExecutionPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ExecutionPayload::Page { mime: data, start } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("source", "page")?;
                map.serialize_entry("data", "text/plain")?;
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

impl From<ExecutionReply> for JupiterContent {
    fn from(value: ExecutionReply) -> Self {
        JupiterContent::ExecutionReply(Box::new(value))
    }
}

impl ExecutionRequest {
    pub fn as_reply(&self, success: bool, count: u32) -> ExecutionReply {
        ExecutionReply {
            success,
            execution_count: count,
            payload: vec![
                // ExecutionPayload::Page { mime: "".to_string(), start: 1 },
                // ExecutionPayload::NextInput { text: "all".to_string(), replace: false },
                // ExecutionPayload::NextInput { text: "other".to_string(), replace: false },
            ],
        }
    }
    pub fn as_result<M, T>(&self, mime: M, data: T) -> JupyterResult<ExecutionResult>
    where
        T: Serialize,
        M: ToString,
    {
        let mut dict = serde_json::Map::new();
        dict.insert(mime.to_string(), serde_json::to_value(data)?);
        println!("data: {:?}", dict);
        Ok(ExecutionResult {
            execution_count: 0,
            data: dict,
            metadata: serde_json::Map::new(),
            transient: serde_json::Map::new(),
        })
    }
}

impl ExecutionResult {
    pub fn with_count(mut self, count: u32) -> ExecutionResult {
        self.execution_count = count;
        self
    }
    pub fn with_data<S, T>(mut self, mime: S, data: T) -> JupyterResult<ExecutionResult>
    where
        T: Serialize,
        S: ToString,
    {
        self.data.insert(mime.to_string(), serde_json::to_value(data)?);
        println!("data: {:?}", self.data);
        Ok(self)
    }
    pub fn with_metadata<T>(mut self, mime: &str, data: T) -> JupyterResult<ExecutionResult>
    where
        T: Serialize,
    {
        self.metadata.insert(mime.to_string(), serde_json::to_value(data)?);
        Ok(self)
    }
    pub fn with_transient<T>(mut self, mime: &str, data: T) -> JupyterResult<ExecutionResult>
    where
        T: Serialize,
    {
        self.transient.insert(mime.to_string(), serde_json::to_value(data)?);
        Ok(self)
    }
}
