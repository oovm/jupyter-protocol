use clap_derive::Parser;
mod install;
mod open_jupyter;
mod start;
mod uninstall;
pub use self::{
    install::InstallAction,
    open_jupyter::OpenAction,
    start::{KernelControl, StartAction},
    uninstall::UninstallAction,
};
use crate::{
    commands::uninstall::get_kernel_dir,
    connection::{KERNEL_JS, LINT_CSS, LINT_JS, LINT_LICENSE},
    JupyterResult, JupyterServerProtocol,
};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{io::Write, path::Path};
