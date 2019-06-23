use clap_derive::Parser;
use evcxr::JupyterResult;
mod install;
mod open_jupyter;
mod start;
mod uninstall;
pub use self::{install::InstallAction, open_jupyter::OpenAction, start::StartAction, uninstall::UninstallAction};
use crate::legacy_install;
