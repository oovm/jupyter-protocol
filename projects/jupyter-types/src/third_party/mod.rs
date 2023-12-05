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
mod for_number;
mod for_string;
#[cfg(feature = "svg")]
mod for_svg;

use crate::{Executed, JupyterContext};
use serde_json::Value;

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
