use super::*;

use evcxr::{json_from_str, JupyterResult};
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

#[derive(Parser)]
pub struct StartAction {
    #[arg(short = 'c', long = "control-file")]
    control_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelControl {
    pub(crate) control_port: u16,
    pub(crate) shell_port: u16,
    pub(crate) stdin_port: u16,
    pub(crate) hb_port: u16,
    pub(crate) iopub_port: u16,
    pub(crate) transport: String,
    pub(crate) ip: String,
    pub(crate) key: String,
}

impl StartAction {
    pub fn run(&self) -> JupyterResult<()> {
        let control_file = PathBuf::from(&self.control_file).canonicalize()?;
        println!("Starting jupyter kernel with control file: {:?}", control_file);
        if let Err(error) = legacy_install::update_if_necessary() {
            eprintln!("Warning: tried to update client, but failed: {}", error);
        }
        let config = KernelControl::parse_control_file(&control_file)?;
        crate::core::Server::run(&config)?;
        Ok(())
    }
}

impl KernelControl {
    fn parse_control_file(file_name: &Path) -> JupyterResult<KernelControl> {
        let control_file = read_to_string(file_name)?;
        let object = json_from_str(&control_file)?;
        Ok(object)
    }
}
