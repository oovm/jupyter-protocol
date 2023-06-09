#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod client;
mod commands;
mod connection;
mod errors;
mod executor;
pub mod helper;
mod jupyter_message;
pub mod value_type;

pub use async_trait::async_trait;

pub(crate) use crate::jupyter_message::*;
#[allow(deprecated)]
pub use crate::{
    commands::{InstallAction, OpenAction, StartAction, UninstallAction},
    errors::{JupyterError, JupyterErrorKind, JupyterResult},
    executor::{
        execution_reply::{ExecutionPayload, ExecutionReply},
        sockets::JupyterKernelSockets,
        Executed, JupyterKernelProtocol, JupyterTheme, LanguageInfo,
    },
    jupyter_message::{ExecutionRequest, ExecutionResult},
};
pub use serde::Serialize;
pub use serde_json::{to_value, Value};
pub use tokio::sync::mpsc::UnboundedSender;
