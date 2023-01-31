use super::*;

/// A trait for types that can be displayed in the Jupyter notebook.
#[derive(Debug)]
pub struct DisplayString {
    text: String,
}

impl Executed for DisplayString {
    fn mime_type(&self) -> String {
        "text/plaintext".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        Value::String(self.text.clone())
    }
}
