pub mod execution_reply;
pub mod sockets;

use crate::{
    executor::sockets::JupyterConnection,
    value_type::{InspectModule, InspectVariable, InspectVariableRequest, JupyterContext},
    ExecutionReply, ExecutionRequest, ExecutionResult, JupyterError, JupyterResult,
};
use serde_json::Value;
use std::{
    fmt::{Debug, Formatter},
    future::Future,
    sync::Arc,
};
use tokio::sync::Mutex;

/// The protocol of the kernel
#[allow(unused_variables)]
pub trait JupyterKernelProtocol: Send + Sync + 'static {
    /// Get the language info of the kernel.
    fn language_info(&self) -> LanguageInfo;

    /// Send
    fn connected(&mut self, context: JupyterConnection);

    /// since Generator is not stable, we use sender instead
    ///
    /// `Generator<Yield = dyn Executed, Return = ExecutionReply>`
    fn running(&mut self, code: ExecutionRequest) -> impl Future<Output = ExecutionReply> + Send;

    /// Show the running time of the code.
    ///
    /// - unit: seconds
    ///
    /// *You can suppress statistics by returning `String::new()` directly, which will not cause heap allocation.*
    fn running_time(&self, time: f64) -> String {
        format!("<sub>Elapsed time: {:.2} seconds.</sub>", time)
    }

    /// Inspect the variables on right side.
    ///
    /// # Arguments
    ///
    /// - `parent`: The variable id provided previously, see [`InspectVariable::variables_reference`].
    ///   - If it is empty, it is a root query.
    ///
    /// # Examples
    fn inspect_variables(&self, parent: Option<InspectVariableRequest>) -> Vec<InspectVariable> {
        vec![InspectVariable::new("inspect_variables").with_type("Unimplemented").with_key(1)]
    }

    /// View and render an object's value
    fn inspect_details(&self, parent: &InspectVariable) -> Box<dyn Executed> {
        // TODO: Replace with `impl Executed` when <https://github.com/rust-lang/rust/issues/91611> stable
        Box::new(JupyterError::custom("`JupyterKernelProtocol::inspect_details` is not yet implemented."))
    }

    /// Query the currently loaded modules
    ///
    /// # Arguments
    ///
    /// * `total`: The number of modules to be loaded, 0 means unlimited.
    fn inspect_modules(&self, total: usize) -> Vec<InspectModule> {
        vec![InspectModule {
            id: 0,
            name: "inspect_modules".to_string(),
            path: "JupyterKernelProtocol::inspect_modules".to_string(),
        }]
    }

    /// Query the currently loaded modules
    ///
    /// # Arguments
    ///
    /// * `total`: The number of modules to be loaded, 0 means unlimited.
    fn inspect_sources(&self) -> String {
        "`JupyterKernelProtocol::inspect_sources` is not yet implemented.".to_string()
    }

    /// Query the currently loaded modules
    ///
    /// # Arguments
    ///
    /// * `total`: The number of modules to be loaded, 0 means unlimited.
    fn interrupt_kernel(&self) -> Option<String> {
        None
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
