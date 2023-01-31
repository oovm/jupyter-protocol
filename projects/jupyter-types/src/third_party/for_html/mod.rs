use crate::{Executed, JupyterContext, JupyterTheme};
use serde_json::Value;
pub mod keywords;

pub mod numbers;
pub mod strings;

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
