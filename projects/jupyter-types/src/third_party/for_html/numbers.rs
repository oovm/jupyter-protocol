use super::*;

/// A trait for types that can be displayed in the Jupyter notebook.
#[derive(Debug)]
pub struct DisplayNumber {
    hint: String,
    text: String,
}

impl Executed for DisplayNumber {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, context: &JupyterContext) -> Value {
        let color = match context.theme {
            JupyterTheme::Light => "#986801",
            JupyterTheme::Dark => "#986801",
        };
        let mut buffer = format!(r#"<span style="color: {color}">{}</span>"#, self.text);
        if !self.hint.is_empty() {
            buffer.push_str(&format!(r#"<span style="color: {color}">{}</span>"#, self.hint));
        }
        Value::String(buffer)
    }
}

impl DisplayNumber {
    /// Create a new display number.
    pub fn new<S: ToString>(text: S) -> Self {
        Self { hint: String::new(), text: text.to_string() }
    }
    /// Create a new display number with a hint.
    pub fn hinted<T, S>(text: T, r#type: S) -> Self
    where
        T: ToString,
        S: ToString,
    {
        Self { hint: r#type.to_string(), text: text.to_string() }
    }
}
