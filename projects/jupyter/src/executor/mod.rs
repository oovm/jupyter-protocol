use crate::{ExecutionReply, ExecutionRequest, ExecutionResult};
use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;

pub mod execution_reply;
pub mod sockets;

/// A executed result that can be render in jupyter notebook.
pub trait Executed: Send {
    /// The mime type of the result.
    fn mime_type(&self) -> String;
    /// Convert the result to json.
    fn as_json(&self, theme: JupyterTheme) -> Value;
}

/// The theme of the Jupyter notebook
#[derive(Copy, Debug, Clone)]
pub enum JupyterTheme {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
}

/// The protocol of the kernel
#[async_trait]
#[allow(unused_variables)]
pub trait JupyterKernelProtocol: Send + Sync + 'static {
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
    /// Bind the execution socket, recommended to use [JupyterKernelSockets](crate::JupyterKernelSockets).
    async fn bind_execution_socket(&self, sender: UnboundedSender<ExecutionResult>) {
        // sink socket, do nothing
    }
}

/// The language information and abilities provided by the kernel
#[derive(Clone, Debug)]
pub struct LanguageInfo {
    /// Language key
    pub language_key: String,
    /// Language display name
    pub language: String,
    /// Language version
    pub version: String,
    /// The 64×64 png logo of the language
    pub png_64: &'static [u8],
    /// The 32×32 png logo of the language
    pub png_32: &'static [u8],
    /// The file extensions of the language
    ///
    /// e.g. `*.rs; *.rsx`
    pub file_extensions: String,
    /// The mimetype of the language
    pub mimetype: String,
    /// One of valid name in <https://pygments.org/docs/lexers>
    ///
    /// Note that you should use the **Short Name**!
    pub lexer: String,
    /// One of valid name in <https://codemirror.net/5/mode/index.html>
    pub highlighter: String,
    /// Notebook exporter
    pub exporter: String,
}

impl LanguageInfo {
    /// Create a new language with the language key and language name
    pub fn new<T, S>(key: T, display: S) -> Self
    where
        T: ToString,
        S: ToString,
    {
        Self {
            language: display.to_string(),
            version: "1.0.0".to_string(),
            png_64: &[],
            png_32: &[],
            language_key: key.to_string(),
            file_extensions: "*.rs".to_string(),
            mimetype: "text/rust".to_string(),
            lexer: "rust".to_string(),
            highlighter: "rust".to_string(),
            exporter: "rust".to_string(),
        }
    }
    /// Set the language file extensions and mimetype
    pub fn with_file_extensions<T, S>(mut self, extension: T, mime: S) -> Self
    where
        T: ToString,
        S: ToString,
    {
        self.file_extensions = extension.to_string();
        self.mimetype = mime.to_string();
        self
    }
    /// Set the language syntax, find lexer name in <https://pygments.org/docs/lexers> and
    /// highlighter name in <https://codemirror.net/5/mode/index.html>
    pub fn with_syntax<T, S>(mut self, lexer: T, highlighter: S) -> Self
    where
        T: ToString,
        S: ToString,
    {
        self.lexer = lexer.to_string();
        self.mimetype = highlighter.to_string();
        self
    }
    /// Set the implement version, recommend to use `env!("CARGO_PKG_VERSION")`
    pub fn with_version<T>(mut self, version: T) -> Self
    where
        T: ToString,
    {
        self.version = version.to_string();
        self
    }
}
