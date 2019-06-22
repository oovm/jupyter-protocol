use super::*;
use std::process::Command;

#[derive(Parser)]
pub struct OpenAction {}

impl OpenAction {
    pub fn run(&self) -> JupyterResult<()> {
        Command::new("jupyter-lab").spawn().expect("jupyter-lab command failed to start");
        Ok(())
    }
}
