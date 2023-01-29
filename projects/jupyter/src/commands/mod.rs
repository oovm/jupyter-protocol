use clap_derive::Parser;
pub mod install;
pub mod open_jupyter;
pub mod start;
pub mod uninstall;
pub use self::{install::InstallAction, open_jupyter::OpenAction, start::StartAction, uninstall::UninstallAction};
use crate::{
    commands::uninstall::get_kernel_dir,
    connection::{KERNEL_JS, LINT_CSS, LINT_JS, LINT_LICENSE},
    JupyterKernelProtocol, JupyterResult,
};
use serde::Serialize;
use std::{io::Write, path::Path};
