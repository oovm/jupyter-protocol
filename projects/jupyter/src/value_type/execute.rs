/// The running context of the Jupyter notebook
#[derive(Copy, Debug, Clone)]
pub struct JupyterContext {
    /// The theme of the Jupyter notebook
    pub theme: JupyterTheme,
}

/// The theme of the Jupyter notebook
#[derive(Copy, Debug, Clone)]
pub enum JupyterTheme {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
}

impl Default for JupyterContext {
    fn default() -> Self {
        Self { theme: JupyterTheme::Light }
    }
}
