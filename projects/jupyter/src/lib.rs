#![doc = include_str!("../readme.md")]

mod client;
mod commands;
mod connection;
mod errors;
mod executor;
pub mod helper;
mod jupyter_message;
mod legacy_install;

pub use async_trait::async_trait;

#[allow(deprecated)]
pub use crate::{
    commands::*,
    errors::{JupyterError, JupyterErrorKind, JupyterResult},
    executor::{
        execution_reply::{ExecutionPayload, ExecutionReply},
        sockets::JupyterServerSockets,
        Executed, JupyterServerProtocol, LanguageInfo,
    },
    jupyter_message::*,
};
pub use serde::Serialize;
pub use serde_json::{to_value, Value};
pub use tokio::sync::mpsc::UnboundedSender;
