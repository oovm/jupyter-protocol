#[cfg(feature = "url")]
pub use url::Url;
#[cfg(feature = "image")]
mod for_image;
#[cfg(feature = "mathml-core")]
mod for_mathml;
use crate::{Executed, JupyterError, JupyterTheme};
#[cfg(feature = "mathml-core")]
pub use mathml_core::MathML;
use serde_json::Value;
#[cfg(feature = "svg")]
use svg::Document;

/// A latex text that can render by [MathJax](https://www.mathjax.org/).
pub struct LatexText {
    text: String,
}

impl LatexText {
    pub fn new<S: ToString>(text: S) -> Self {
        LatexText { text: text.to_string() }
    }
}

impl Executed for JupyterError {
    fn mime_type(&self) -> String {
        "application/vnd.jupyter.stderr".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        format!("<div class=\"error\">{}</div>", self).into()
    }
}

impl Executed for bool {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        // bool not support in Jupyter
        Value::String(self.to_string())
    }
}

impl Executed for String {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        self.clone().into()
    }
}

impl Executed for char {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }
    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(self.to_string())
    }
}

impl<'a> Executed for &'a str {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }
    fn as_json(&self, _: JupyterTheme) -> Value {
        self.to_string().into()
    }
}

impl Executed for Value {
    fn mime_type(&self) -> String {
        "application/json".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        self.clone()
    }
}

#[cfg(feature = "url")]
impl Executed for Url {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(format!(r#"<a href="{}">{}</a>"#, self, self))
    }
}

impl Executed for i32 {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        // number not support in Jupyter
        Value::String(self.to_string())
    }
}

impl Executed for f64 {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(self.to_string())
    }
}

impl Executed for LatexText {
    fn mime_type(&self) -> String {
        "text/latex".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(self.text.clone())
    }
}

#[cfg(feature = "mathml-core")]
impl Executed for MathML {
    fn mime_type(&self) -> String {
        // has been banned, https://github.com/gnestor/notebook/blob/master/notebook/static/notebook/js/outputarea.js#L260
        // "application/mathml+xml".to_string();
        "text/html".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(self.to_string())
    }
}

#[cfg(feature = "svg")]
impl Executed for Document {
    fn mime_type(&self) -> String {
        "image/svg+xml".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(self.to_string())
    }
}
