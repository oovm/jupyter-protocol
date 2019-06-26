use super::*;
use crate::control_file;
use std::path::PathBuf;

#[derive(Parser)]
pub struct StartAction {
    #[arg(short = 'c', long = "control-file")]
    control_file: String,
}

impl StartAction {
    pub fn run(&self) -> JupyterResult<()> {
        let control_file = PathBuf::from(&self.control_file).canonicalize()?;
        println!("Starting jupyter kernel with control file: {:?}", control_file);
        if let Err(error) = legacy_install::update_if_necessary() {
            eprintln!("Warning: tried to update client, but failed: {}", error);
        }
        let config = control_file::Control::parse_file(&self.control_file)?;
        crate::core::Server::run(&config)?;
        Ok(())
    }
}
