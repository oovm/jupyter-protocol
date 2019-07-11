use super::*;
use std::path::PathBuf;

#[derive(Parser)]
pub struct UninstallAction {}

impl UninstallAction {
    pub fn run(&self) -> JupyterResult<()> {
        let kernel_dir = get_kernel_dir()?;
        println!("Deleting {}", kernel_dir.to_string_lossy());
        std::fs::remove_dir_all(kernel_dir)?;
        println!("Uninstall complete");
        Ok(())
    }
}

// https://jupyter-client.readthedocs.io/en/latest/kernels.html
pub fn get_kernel_dir() -> JupyterResult<PathBuf> {
    let jupyter_dir = if let Ok(dir) = std::env::var("JUPYTER_PATH") {
        PathBuf::from(dir)
    }
    else if let Some(dir) = get_user_kernel_dir() {
        dir
    }
    else {
        panic!("Couldn't get XDG data directory");
    };
    Ok(jupyter_dir.join("kernels").join("rust"))
}

#[cfg(not(target_os = "macos"))]
fn get_user_kernel_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|data_dir| data_dir.join("jupyter"))
}

#[cfg(target_os = "macos")]
fn get_user_kernel_dir() -> Option<PathBuf> {
    dirs::data_dir().and_then(|d| d.parent().map(|data_dir| data_dir.join("Jupyter")))
}
