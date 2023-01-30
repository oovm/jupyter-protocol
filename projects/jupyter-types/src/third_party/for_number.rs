use crate::{Executed, JupyterContext};
use serde_json::Value;

impl Executed for bool {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        // bool not support in Jupyter
        Value::String(self.to_string())
    }
}

impl Executed for i32 {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        // number not support in Jupyter
        Value::String(self.to_string())
    }
}

impl Executed for f64 {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        Value::String(self.to_string())
    }
}
