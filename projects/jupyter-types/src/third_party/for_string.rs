use crate::{Executed, JupyterContext};
use serde_json::Value;

impl Executed for char {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }
    fn as_json(&self, _: &JupyterContext) -> Value {
        Value::String(self.to_string())
    }
}

impl<'a> Executed for &'a str {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }
    fn as_json(&self, _: &JupyterContext) -> Value {
        self.to_string().into()
    }
}

impl Executed for String {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        self.clone().into()
    }
}
