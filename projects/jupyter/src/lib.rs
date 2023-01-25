#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

extern crate core;

mod client;
mod commands;
mod connection;
mod errors;
mod executor;
pub(crate) mod jupyter_message;
pub mod value_type;

pub use crate::jupyter_message::JupyterMessage;
#[allow(deprecated)]
pub use crate::{
    commands::{InstallAction, OpenAction, StartAction, UninstallAction},
    errors::{JupyterError, JupyterErrorKind, JupyterResult},
    executor::{
        execution_reply::{ExecutionPayload, ExecutionReply},
        sockets::{JupyterConnection, JupyterKernelSockets, JupyterStream},
        Executed, JupyterKernelProtocol, LanguageInfo,
    },
    jupyter_message::{ExecutionRequest, ExecutionResult},
};
pub use serde::Serialize;
pub use serde_json::{to_value, Value};
pub use tokio::sync::mpsc::UnboundedSender;
