use crate::JupyterResult;
use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait ExecuteContext {
    fn language_info(&self) -> LanguageInfo;

    async fn run(&mut self, code: &str, count: i32) -> JupyterResult<ExecuteResult>;
}

pub enum ExecuteResult {
    Success,
    Error,
}

pub struct LanguageInfo {
    pub language: String,
    pub file_extensions: String,
}

pub struct SinkExecutor {
    pub name: String,
}
#[async_trait]
impl ExecuteContext for SinkExecutor {
    fn language_info(&self) -> LanguageInfo {
        LanguageInfo { language: "Rust".to_string(), file_extensions: ".rs".to_string() }
    }

    async fn run(&mut self, code: &str, count: i32) -> JupyterResult<ExecuteResult> {
        todo!()
    }
}
