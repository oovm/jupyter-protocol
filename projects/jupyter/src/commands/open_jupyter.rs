use super::*;
use std::process::Command;

/// Open the jupyter-lab application for debug.
#[derive(Clone, Debug, Parser)]
pub struct OpenAction {
    /// Open path of jupyter-lab.
    path: Option<String>,
}

impl OpenAction {
    /// Run the jupyter-lab application.
    pub fn run(&self) -> JupyterResult<()> {
        Command::new("python").args(&["-m", "jupyterlab"]).spawn().expect("jupyter-lab command failed to start");
        Ok(())
    }
}
