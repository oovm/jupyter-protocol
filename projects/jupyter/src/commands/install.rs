use super::*;

#[derive(Parser)]
pub struct InstallAction {
    /// Optional name to operate on
    name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KernelConfig {
    argv: Vec<String>,
    display_name: String,
    language: String,
    interrupt_mode: String,
}

impl InstallAction {
    pub fn run(&self) -> JupyterResult<()> {
        legacy_install::install();
        Ok(())
    }
}

impl KernelConfig {
    pub fn new(language: &str, display: &str) -> JupyterResult<Self> {
        match std::env::current_exe() {
            Ok(path) => Ok(Self {
                argv: vec![
                    path.to_string_lossy().to_string(),
                    "start".to_string(),
                    "--control-file".to_string(),
                    "{connection_file}".to_string(),
                ],
                display_name: display.to_string(),
                language: language.to_string(),
                interrupt_mode: "message".to_string(),
            }),
            Err(e) => {
                // "current exe path isn't valid UTF-8"
                panic!("Couldn't get current exe path: {}", e);
            }
        }
    }
}
