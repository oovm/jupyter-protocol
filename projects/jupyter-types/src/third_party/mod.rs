#[cfg(feature = "image")]
mod for_image;
#[cfg(feature = "mathml-core")]
mod for_mathml;
#[cfg(feature = "ndarray")]
mod for_ndarray;
#[cfg(feature = "mathml-core")]
pub use mathml_core::MathML;
#[cfg(feature = "ndarray")]
pub use ndarray::{Array1, Array2};

#[cfg(feature = "svg")]
pub use svg::Document;
#[cfg(feature = "url")]
pub use url::Url;

#[cfg(feature = "svg")]
mod for_svg;

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

impl Executed for String {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        self.clone().into()
    }
}

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

impl Executed for Value {
    fn mime_type(&self) -> String {
        "application/json".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        self.clone()
    }
}

#[cfg(feature = "url")]
impl Executed for Url {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        Value::String(format!(r#"<a href="{}">{}</a>"#, self, self))
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
