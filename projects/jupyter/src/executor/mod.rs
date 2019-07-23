use crate::JupyterResult;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ExecuteContext {
    fn language_info(&self) -> LanguageInfo;

    async fn run(&mut self, code: &str) -> JupyterResult<Value>;
}


pub struct LanguageInfo {
    pub language: String,
    pub file_extensions: String,
}

pub struct SinkExecutor {
    name: String,
}

impl Default for SinkExecutor {
    fn default() -> Self {
        Self { name: "sink".to_string() }
    }
}

#[async_trait]
impl ExecuteContext for SinkExecutor {
    fn language_info(&self) -> LanguageInfo {
        LanguageInfo { language: "Rust".to_string(), file_extensions: ".rs".to_string() }
    }

    async fn run(&mut self, code: &str) -> JupyterResult<Value> {
        todo!()
    }
}
