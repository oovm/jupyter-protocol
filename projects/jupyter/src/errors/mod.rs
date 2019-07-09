// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use ariadne::{Color, ColorGenerator, Label, Report, ReportKind};
use serde_derive::{Deserialize, Serialize};

use hex::FromHexError;
use serde_json::Value;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Write as _},
    ops::Range,
};
use zeromq::ZmqError;

pub type JupyterResult<T> = Result<T, JupyterError>;

#[derive(Debug, Clone)]
pub struct JupyterError {
    kind: Box<JupyterErrorKind>,
}

impl JupyterError {
    pub fn missing_field(field: &'static str) -> Self {
        Self { kind: Box::new(JupyterErrorKind::MissingField(field)) }
    }
}

#[derive(Debug, Clone)]
pub enum JupyterErrorKind {
    CompilationErrors(Vec<CompilationError>),
    TypeRedefinedVariablesLost(Vec<String>),
    Message(String),
    MissingField(&'static str),
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
        // match self {
        //     JupyterErrorKind::CompilationErrors(errors) => {
        //         for error in errors {
        //             write!(f, "{}", error.message())?;
        //         }
        //     }
        //     JupyterErrorKind::TypeRedefinedVariablesLost(variables) => {
        //         write!(f, "A type redefinition resulted in the following variables being lost: {}", variables.join(", "))?;
        //     }
        //     JupyterErrorKind::Message(message) | JupyterErrorKind::SubprocessTerminated(message) => write!(f, "{message}")?,
        // }
        todo!();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CompilationError {
    message: String,
    pub json: serde_json::Value,
    pub(crate) code_origins: Vec<CodeKind>,
    spanned_messages: Vec<SpannedMessage>,
    spanned_helps: Vec<SpannedMessage>,
    level: String,
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

fn span_to_byte_range(source: &str, span: &Span) -> Range<usize> {
    fn line_and_number_to_byte_offset(source: &str, line_number: usize, column: usize) -> usize {
        source.lines().take(line_number - 1).map(|x| x.len()).sum::<usize>() + column + line_number - 2
    }
    line_and_number_to_byte_offset(source, span.start_line, span.start_column)
        ..line_and_number_to_byte_offset(source, span.end_line, span.end_column)
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

impl From<std::str::Utf8Error> for JupyterError {
    fn from(error: std::str::Utf8Error) -> Self {
        JupyterError { kind: Box::new(JupyterErrorKind::Message(error.to_string())) }
    }
}

impl From<()> for JupyterError {
    fn from(error: ()) -> Self {
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
