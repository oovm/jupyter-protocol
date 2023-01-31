use crate::{Executed, JupyterContext};
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
impl Executed for LatexText {
    fn mime_type(&self) -> String {
        "text/latex".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        Value::String(self.text.clone())
    }
}
