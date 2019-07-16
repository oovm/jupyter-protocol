use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub code: String,
    pub execution_count: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionReply {
    execution_count: i32,
    data: Value,
    metadata: Value,
}

impl From<ExecutionReply> for JupiterContent {
    fn from(value: ExecutionReply) -> Self {
        JupiterContent::ExecutionReply(Box::new(value))
    }
}

impl ExecutionRequest {
    pub fn as_reply<T>(&self, data: T) -> JupyterResult<ExecutionReply>
    where
        T: Serialize,
    {
        Ok(ExecutionReply { execution_count: self.execution_count, data: serde_json::to_value(data)?, metadata: Value::Null })
    }
    pub fn as_error(&self) {
        unimplemented!()
    }
}

impl ExecutionReply {
    pub fn with_meta<T>(self, data: T) -> JupyterResult<ExecutionReply>
    where
        T: Serialize,
    {
        Ok(ExecutionReply { execution_count: self.execution_count, data: self.data, metadata: serde_json::to_value(data)? })
    }
}
