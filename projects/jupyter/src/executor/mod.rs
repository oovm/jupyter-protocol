use std::vec::IntoIter;
use crate::{ExecutionRequest, JupyterResult};
use async_trait::async_trait;
use image::RgbaImage;
use serde_json::{to_value, Value};
use crate::helper::bytes_to_png;

pub trait Executed: Send {
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

impl Executed for Value {
    fn mime_type(&self) -> String {
        "application/json".to_string()
    }

    fn as_json(&self) -> Value {
        self.clone()
    }
}

impl Executed for f64 {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self) -> Value {
        Value::Number(serde_json::Number::from_f64(*self).unwrap_or(serde_json::Number::from(0)))
    }
}

#[async_trait]
#[allow(unused_variables)]
pub trait ExecuteContext {
    type Executed: Executed;

    fn logo(&self) -> RgbaImage;

    fn language_info(&self) -> LanguageInfo;

    async fn running(&mut self, code: ExecutionRequest) -> Vec<Self::Executed>;

    /// Show the running time of the code, return nil if not supported
    ///
    /// - unit: seconds
    fn running_time(&self, time: f64) -> String {
        format!("<sub>Elapsed time: {:.2} seconds.</sub>", time)
    }
}

pub struct LanguageInfo {
    /// Language display
    pub language: String,
    /// Language key
    pub png_64: &'static [u8],
    pub png_32: &'static [u8],
    pub language_key: String,
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
    type Executed = Value;

    fn logo(&self) -> RgbaImage {
        bytes_to_png(include_bytes!("../../third_party/rust/rust-logo-64x64.png")).expect("Failed to decode rust logo")
    }

    fn language_info(&self) -> LanguageInfo {
        LanguageInfo { language: "Rust".to_string(), png_64: include_bytes!("../../third_party/rust/rust-logo-32x32.png"), png_32: include_bytes!("../../third_party/rust/rust-logo-64x64.png"), language_key: "rust".to_string(), file_extensions: ".rs".to_string() }
    }

    async fn running(&mut self, code: ExecutionRequest) -> Vec<Self::Executed> {
        vec![to_value(code).unwrap_or(Value::Null)]
    }
    fn running_time(&self, _: f64) -> String {
        String::new()
    }
}
