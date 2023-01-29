#![doc = include_str!("readme.md")]

mod inspects;

pub use self::inspects::{InspectModule, InspectVariable, InspectVariableRequest};
use crate::JupyterError;
pub use jupyter_types::{Executed, JupyterContext, JupyterTheme};
use serde_json::Value;

/// A latex text that can render by [MathJax](https://www.mathjax.org/).
#[derive(Clone, Debug)]
pub struct LatexText {
    text: String,
}
impl LatexText {
    /// Create a new latex text.
    pub fn new<S: ToString>(text: S) -> Self {
        LatexText { text: text.to_string() }
    }
}

/// A raw html text.
#[derive(Clone, Debug)]
pub struct HtmlText {
    text: String,
}

impl HtmlText {
    /// Create a new html text.
    pub fn new<S: ToString>(text: S) -> Self {
        HtmlText { text: text.to_string() }
    }
}
impl Executed for HtmlText {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        self.text.clone().into()
    }
}

impl Executed for JupyterError {
    fn mime_type(&self) -> String {
        "application/vnd.jupyter.stderr".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        Value::String(self.to_string())
    }
}

impl Executed for LatexText {
    fn mime_type(&self) -> String {
        "text/latex".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        Value::String(self.text.clone())
    }
}
