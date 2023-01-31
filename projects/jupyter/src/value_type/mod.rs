#![doc = include_str!("readme.md")]

mod inspects;

pub use self::inspects::{InspectModule, InspectVariable, InspectVariableRequest};
use crate::JupyterError;
pub use jupyter_types::{Executed, JupyterContext, JupyterTheme};
use serde_json::Value;

impl Executed for JupyterError {
    fn mime_type(&self) -> String {
        "application/vnd.jupyter.stderr".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        Value::String(self.to_string())
    }
}
