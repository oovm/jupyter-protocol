use crate::{Executed, JupyterContext};
use mathml_core::MathML;
use serde_json::Value;
#[cfg(feature = "mathml-core")]
impl Executed for MathML {
    fn mime_type(&self) -> String {
        // has been banned, https://github.com/gnestor/notebook/blob/master/notebook/static/notebook/js/outputarea.js#L260
        // "application/mathml+xml".to_string();
        "text/html".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        Value::String(self.to_string())
    }
}
