use crate::{ExecutionReply, ExecutionRequest, ExecutionResult};
use async_trait::async_trait;
use serde_json::{to_value, Value};
use tokio::sync::mpsc::UnboundedSender;

pub mod sockets;
mod value_type;

pub trait Executed: Send {
    fn mime_type(&self) -> String;
    fn as_json(&self) -> Value;
}


#[async_trait]
#[allow(unused_variables)]
pub trait JupyterServerProtocol: Send + Sync + 'static {
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

    /// Bind the execution socket, recommended to use [JupyterServerSockets].
    async fn bind_execution_socket(&self, sender: UnboundedSender<ExecutionResult>) {
        // sink socket, do nothing
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
