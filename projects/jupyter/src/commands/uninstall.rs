use super::*;

#[derive(Parser)]
pub struct UninstallAction {
    /// Optional name to operate on
    name: Option<String>,
}
impl UninstallAction {
    pub fn run(&self) -> JupyterResult<()> {
        legacy_install::uninstall();
        Ok(())
    }
}
