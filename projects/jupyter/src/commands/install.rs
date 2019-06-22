use super::*;

#[derive(Parser)]
pub struct InstallAction {
    /// Optional name to operate on
    name: Option<String>,
}

impl InstallAction {
    pub fn run(&self) -> JupyterResult<()> {
        legacy_install::install();
        Ok(())
    }
}
