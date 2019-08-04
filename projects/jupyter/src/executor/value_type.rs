use super::*;
use crate::JupyterError;

impl Executed for JupyterError {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_data(&self) -> Value {
        format!("<div class=\"error\">{}</div>", self).into()
    }
}

impl Executed for String {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_data(&self) -> Value {
        Value::String(self.clone())
    }
}

impl Executed for Value {
    fn mime_type(&self) -> String {
        "application/json".to_string()
    }

    fn as_data(&self) -> Value {
        self.clone()
    }
}

impl Executed for f64 {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_data(&self) -> Value {
        Value::Number(serde_json::Number::from_f64(*self).unwrap_or(serde_json::Number::from(0)))
    }
}
