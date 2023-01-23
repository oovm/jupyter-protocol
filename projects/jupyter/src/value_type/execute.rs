/// The running context of the Jupyter notebook
#[derive(Copy, Debug, Clone)]
pub struct JupyterContext {
    /// The theme of the Jupyter notebook
    pub theme: JupyterTheme,
    /// Limit the number of output lists to prevent the front end from getting stuck
    pub record_limit: usize,
    /// Limit the number of output objects to prevent the front end from getting stuck
    pub object_limit: usize,
    /// Limit the depth of output objects to prevent the front end from getting stuck
    pub object_depth: usize,
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
        Self { theme: JupyterTheme::Light, record_limit: 64, object_limit: 64, object_depth: 64 }
    }
}
