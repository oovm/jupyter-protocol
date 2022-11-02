use hex::FromHexError;

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    str::Utf8Error,
};
use tokio::{
    sync::{mpsc::error::SendError, TryLockError},
    task::JoinError,
};
use zeromq::ZmqError;

/// The result type for Jupyter.
pub type JupyterResult<T> = Result<T, JupyterError>;

/// The error type for Jupyter.
#[derive(Debug, Clone)]
pub struct JupyterError {
    kind: Box<JupyterErrorKind>,
}

impl JupyterError {
    /// Create a [JupyterErrorKind::Custom] error.
    pub fn custom<T: ToString>(message: T) -> Self {
        Self { kind: Box::new(JupyterErrorKind::Custom(message.to_string())) }
    }
    /// Create a [JupyterErrorKind::MissingField] error.
    pub fn missing_field(field: &'static str) -> Self {
        Self { kind: Box::new(JupyterErrorKind::MissingField(field)) }
    }
    /// Create a [JupyterErrorKind::ExceptType] error.
    pub fn except_type(except_type: &'static str) -> Self {
        Self { kind: Box::new(JupyterErrorKind::ExceptType(except_type)) }
    }
    /// Create a [JupyterErrorKind::ChannelBlockage] error.
    pub fn channel_block(channel: &'static str) -> Self {
        Self { kind: Box::new(JupyterErrorKind::ChannelBlockage(channel)) }
    }
}

/// The error kind for Jupyter.
#[derive(Debug, Clone)]
pub enum JupyterErrorKind {
    /// The type redefined, variables lost.
    TypeRedefinedVariablesLost(Vec<String>),
    /// Custom error.
    Custom(String),
    /// Error message.
    Message(String),
    /// Missing field.
    MissingField(&'static str),
    /// Except type.
    ExceptType(&'static str),
    /// Channel blockage.
    ChannelBlockage(&'static str),
    /// Zmq error.
    SubprocessTerminated(String),
}

impl Error for JupyterErrorKind {}
impl Error for JupyterError {}

impl Display for JupyterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl Display for JupyterErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JupyterErrorKind::TypeRedefinedVariablesLost(variables) => {
                writeln!(f, "Type redefined, variables lost:")?;
                for variable in variables {
                    writeln!(f, "{}", variable)?;
                }
                Ok(())
            }
            JupyterErrorKind::Custom(message) => write!(f, "{}", message),
            JupyterErrorKind::Message(message) => write!(f, "{}", message),
            JupyterErrorKind::MissingField(field) => write!(f, "Missing field: {}", field),
            JupyterErrorKind::ExceptType(except_type) => write!(f, "Except type: {}", except_type),
            JupyterErrorKind::ChannelBlockage(channel) => write!(f, "Channel blockage: {}", channel),
            JupyterErrorKind::SubprocessTerminated(message) => write!(f, "Subprocess terminated: {}", message),
        }
    }
}

impl From<JupyterErrorKind> for JupyterError {
    fn from(value: JupyterErrorKind) -> Self {
        JupyterError { kind: Box::new(value) }
    }
}

impl From<std::fmt::Error> for JupyterError {
    fn from(error: std::fmt::Error) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message(error.to_string())) }
    }
}

impl From<std::io::Error> for JupyterError {
    fn from(error: std::io::Error) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message(error.to_string())) }
    }
}

impl From<serde_json::Error> for JupyterError {
    fn from(error: serde_json::Error) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message(error.to_string())) }
    }
}

impl From<Utf8Error> for JupyterError {
    fn from(error: Utf8Error) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message(error.to_string())) }
    }
}

impl From<()> for JupyterError {
    fn from(_: ()) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message("".to_string())) }
    }
}
impl From<FromHexError> for JupyterError {
    fn from(error: FromHexError) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message(error.to_string())) }
    }
}

impl From<ZmqError> for JupyterError {
    fn from(error: ZmqError) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message(error.to_string())) }
    }
}
impl From<JoinError> for JupyterError {
    fn from(error: JoinError) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message(error.to_string())) }
    }
}
impl From<TryLockError> for JupyterError {
    fn from(error: TryLockError) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message(error.to_string())) }
    }
}

impl<T> From<SendError<T>> for JupyterError {
    fn from(value: SendError<T>) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message(value.to_string())) }
    }
}
