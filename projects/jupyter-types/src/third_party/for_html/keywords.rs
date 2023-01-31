use super::*;

/// A trait for types that can be displayed in the Jupyter notebook.
#[derive(Debug)]
pub struct DisplayKeywords {
    text: String,
}

impl Executed for DisplayKeywords {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, context: &JupyterContext) -> Value {
        let color = match context.theme {
            JupyterTheme::Light => "#A626A4",
            JupyterTheme::Dark => "#A626A4",
        };
        Value::String(format!(r#"<span style="color: {color}">{}</span>"#, self.text))
    }
}

impl DisplayKeywords {
    /// Create a new display keywords.
    pub fn new<S: ToString>(text: S) -> Self {
        Self { text: text.to_string() }
    }
}
