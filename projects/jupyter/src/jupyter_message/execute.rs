use super::*;
use crate::ExecutionReply;
use serde::{ser::SerializeStruct, Serializer};
use serde_json::Map;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct ExecutionResult {
    execution_count: u32,
    data: BTreeMap<String, Value>,
    metadata: Map<String, Value>,
    transient: Map<String, Value>,
}

impl Serialize for ExecutionResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ExecutionResult", 4)?;
        state.serialize_field("execution_count", &self.execution_count)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("metadata", &self.metadata)?;
        state.serialize_field("transient", &self.transient)?;
        state.end()
    }
}
impl Default for ExecutionResult {
    fn default() -> Self {
        Self { execution_count: 0, data: BTreeMap::new(), metadata: Map::new(), transient: Map::new() }
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

impl ExecutionRequest {
    pub fn as_reply(&self, success: bool, count: u32) -> ExecutionReply {
        ExecutionReply::new(success, count)
    }
    pub fn as_result(&self, mime: String, data: Value) -> ExecutionResult {
        let mut dict = BTreeMap::new();
        dict.insert(mime.to_string(), data);
        ExecutionResult { execution_count: 0, data: dict, metadata: serde_json::Map::new(), transient: serde_json::Map::new() }
    }
}

impl ExecutionResult {
    pub fn with_count(mut self, count: u32) -> ExecutionResult {
        self.execution_count = count;
        self
    }
    pub fn with_data(mut self, mime: String, data: Value) -> ExecutionResult {
        self.data.insert(mime, data);
        self
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
