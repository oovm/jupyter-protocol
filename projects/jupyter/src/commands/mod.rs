use clap_derive::Parser;
mod install;
mod open_jupyter;
mod start;
mod uninstall;
pub use self::{
    install::{InstallAction, KernelConfig},
    open_jupyter::OpenAction,
    start::{KernelControl, StartAction},
    uninstall::UninstallAction,
};
use crate::{legacy_install, JupyterResult};
use serde_derive::{Deserialize, Serialize};
