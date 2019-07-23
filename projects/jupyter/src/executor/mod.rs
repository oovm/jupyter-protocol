use std::vec::IntoIter;
use crate::{ExecutionRequest, JupyterResult};
use async_trait::async_trait;
use serde_json::Value;

pub trait Executed {
    fn mime_type(&self) -> String;
    fn as_json(&self) -> Value;
}

impl Executed for String {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self) -> Value {
        Value::String(self.clone())
    }
}

#[async_trait]
#[allow(unused_variables)]
pub trait ExecuteContext {
    type Executed: Executed;
    fn language_info(&self) -> LanguageInfo;

    async fn running(&mut self, code: ExecutionRequest) -> Vec<Self::Executed>;

    /// Show the running time of the code, return nil if not supported
    ///
    /// - unit: seconds
    fn running_time(&self, time: f64) -> String {
        format!("<div>Elapsed time: {:.2} seconds.</div>", time)
    }
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
    type Executed = String;

    fn language_info(&self) -> LanguageInfo {
        LanguageInfo { language: "Rust".to_string(), file_extensions: ".rs".to_string() }
    }

    async fn running(&mut self, code: ExecutionRequest) -> Vec<Self::Executed> {
        vec!["Hello, world!".to_string(), "Hello, world2!".to_string()]
    }
}
