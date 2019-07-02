use clap_derive::Parser;
use evcxr::JupyterResult;
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
use crate::legacy_install;
use serde_derive::{Deserialize, Serialize};
