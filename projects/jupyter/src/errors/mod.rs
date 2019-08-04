// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use serde_derive::{Deserialize, Serialize};

use hex::FromHexError;

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Write as _},
    str::Utf8Error,
};
use tokio::{
    sync::{mpsc::error::SendError, TryLockError},
    task::JoinError,
};
use zeromq::ZmqError;

pub type JupyterResult<T> = Result<T, JupyterError>;

#[derive(Debug, Clone)]
pub struct JupyterError {
    kind: Box<JupyterErrorKind>,
}

impl JupyterError {
    pub fn any<T: ToString>(message: T) -> Self {
        Self { kind: Box::new(JupyterErrorKind::Custom(message.to_string())) }
    }
    pub fn missing_field(field: &'static str) -> Self {
        Self { kind: Box::new(JupyterErrorKind::MissingField(field)) }
    }
    pub fn except_type(except_type: &'static str) -> Self {
        Self { kind: Box::new(JupyterErrorKind::ExceptType(except_type)) }
    }
    pub fn channel_block(channel: &'static str) -> Self {
        Self { kind: Box::new(JupyterErrorKind::ChannelBlockage(channel)) }
    }
}

#[derive(Debug, Clone)]
pub enum JupyterErrorKind {
    TypeRedefinedVariablesLost(Vec<String>),
    Custom(String),
    Message(String),
    MissingField(&'static str),
    ExceptType(&'static str),
    ChannelBlockage(&'static str),
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

#[derive(Debug, Clone)]
pub enum CodeKind {
    UserCode,
    MacroExpansion,
    ExternalCrate,
}
#[derive(Debug, Clone)]
pub enum Theme {
    Light,
    Dark,
}

fn sanitize_message(message: &str) -> String {
    // Any references to `evcxr_variable_store` are beyond the end of what the
    // user typed, so we replace such references with something more meaningful.
    // This is mostly helpful with missing semicolons on let statements, which
    // produce errors such as "expected `;`, found `evcxr_variable_store`"
    message.replace("`evcxr_variable_store`", "<end of input>")
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Span {
    /// 1-based line number in the original user code on which the span starts (inclusive).
    pub start_line: usize,
    /// 1-based column (character) number in the original user code on which the span starts
    /// (inclusive).
    pub start_column: usize,
    /// 1-based line number in the original user code on which the span ends (inclusive).
    pub end_line: usize,
    /// 1-based column (character) number in the original user code on which the span ends
    /// (exclusive).
    pub end_column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpannedMessage {
    pub span: Option<Span>,
    /// Output lines relevant to the message.
    pub lines: Vec<String>,
    pub label: String,
    pub is_primary: bool,
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
