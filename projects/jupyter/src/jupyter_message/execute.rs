use super::*;
use crate::ExecutionReply;
use serde_json::Map;

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

impl ExecutionRequest {
    pub fn as_reply(&self, success: bool, count: u32) -> ExecutionReply {
        ExecutionReply::new(success, count)
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
