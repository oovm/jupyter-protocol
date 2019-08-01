#![feature(generator_trait)]
// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod client;
mod commands;
mod connection;
mod errors;
mod executor;
pub mod helper;
mod jupyter_message;
mod legacy_install;

pub use async_trait::async_trait;

pub use crate::{
    commands::*,
    errors::{JupyterError, JupyterErrorKind, JupyterResult},
    executor::{Executed, JupyterServerProtocol, LanguageInfo},
    jupyter_message::*,
};
pub use serde::Serialize;
pub use serde_json::{to_value, Value};
