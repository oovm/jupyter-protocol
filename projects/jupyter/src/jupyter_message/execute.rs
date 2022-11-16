use super::*;
use crate::{Executed, ExecutionReply, JupyterTheme};
use serde::{ser::SerializeStruct, Serializer};
use serde_json::Map;
use std::collections::BTreeMap;

/// The result of executing code
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
/// The request to execute code
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionRequest {
    /// The code to execute
    pub code: String,
    /// Whether to execute the code as quietly as possible
    pub silent: bool,
    /// Whether to store history
    pub store_history: bool,
    /// A mapping of names to expressions to be evaluated in the user's dict.
    pub allow_stdin: bool,
    /// A mapping of names to expressions to be evaluated in the user's dict.
    pub stop_on_error: bool,
    /// A mapping of names to expressions to be evaluated in the user's dict.
    pub user_expressions: Value,
    /// A mapping of names to expressions to be evaluated in the user's dict.
    #[serde(default)]
    pub execution_count: u32,
}

impl ExecutionRequest {
    /// Create a new execution request
    pub fn as_reply(&self, success: bool, count: u32) -> ExecutionReply {
        ExecutionReply::new(success, count)
    }
    /// Create a new execution request
    pub fn as_result(&self, mime: String, data: Value) -> ExecutionResult {
        let mut dict = BTreeMap::new();
        dict.insert(mime.to_string(), data);
        ExecutionResult { execution_count: 0, data: dict, metadata: serde_json::Map::new(), transient: serde_json::Map::new() }
    }
}

impl ExecutionResult {
    /// Create a new execution result
    pub fn new<T>(execute: &T) -> ExecutionResult
    where
        T: Executed + ?Sized,
    {
        let mut data = BTreeMap::new();
        data.insert(execute.mime_type(), execute.as_json(JupyterTheme::Light));
        Self { execution_count: 0, data, metadata: Default::default(), transient: Default::default() }
    }

    /// Create a new execution result
    pub fn with_count(mut self, count: u32) -> ExecutionResult {
        self.execution_count = count;
        self
    }
    /// Create a new execution result
    pub fn with_data(mut self, mime: String, data: Value) -> ExecutionResult {
        self.data.insert(mime, data);
        self
    }
    /// Create a new execution result
    pub fn with_metadata<T>(mut self, mime: &str, data: T) -> JupyterResult<ExecutionResult>
    where
        T: Serialize,
    {
        self.metadata.insert(mime.to_string(), serde_json::to_value(data)?);
        Ok(self)
    }
    /// Create a new execution result
    pub fn with_transient<T>(mut self, mime: &str, data: T) -> JupyterResult<ExecutionResult>
    where
        T: Serialize,
    {
        self.transient.insert(mime.to_string(), serde_json::to_value(data)?);
        Ok(self)
    }
}
