use crate::{Executed, JupyterContext};
use serde_json::Value;
use svg::Document;

#[cfg(feature = "svg")]
impl Executed for Document {
    fn mime_type(&self) -> String {
        "image/svg+xml".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        Value::String(self.to_string())
    }
}
