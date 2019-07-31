use crate::{helper::bytes_to_png, ExecutionReply, ExecutionRequest, JupyterResult};
use async_trait::async_trait;
use image::RgbaImage;
use serde_json::{to_value, Value};
use std::{
    ops::{Generator, GeneratorState},
    vec::IntoIter,
};
mod sockets;
pub use self::sockets::JupyterServerSockets;

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
pub trait JupyterServerProtocol {
    fn language_info(&self) -> LanguageInfo;

    /// since Generator is not stable, we use sender instead
    ///
    /// `Generator<Yield = dyn Executed, Return = ExecutionReply>`
    async fn running(&mut self, code: ExecutionRequest) -> ExecutionReply;

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
