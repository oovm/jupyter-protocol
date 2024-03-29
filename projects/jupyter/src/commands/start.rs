use super::*;

use crate::{client::SealedServer, JupyterKernelProtocol};

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};
/// To start a jupyter kernel for language.
#[derive(Clone, Debug, Parser)]
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
    /// Start a jupyter kernel for language.
    pub fn run<T>(&self, server: T) -> JupyterResult<()>
    where
        T: JupyterKernelProtocol + 'static,
    {
        let control_file = PathBuf::from(&self.control_file).canonicalize()?;
        #[cfg(feature = "url")]
        {
            use jupyter_types::third_party::Url;
            println!("Starting jupyter kernel with control file: {}", Url::from_file_path(&control_file)?);
        }
        // if let Err(error) = legacy_install::update_if_necessary() {
        //     eprintln!("Warning: tried to update client, but failed: {}", error);
        // }
        SealedServer::run(&KernelControl::parse_control_file(&control_file)?, server)?;
        Ok(())
    }
}

impl KernelControl {
    fn parse_control_file(file_name: &Path) -> JupyterResult<KernelControl> {
        let control_file = read_to_string(file_name)?;
        let object = from_str(&control_file)?;
        Ok(object)
    }
}
