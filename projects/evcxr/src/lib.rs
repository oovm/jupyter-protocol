// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(all(unix, not(target_os = "freebsd")))]
#[macro_use]
extern crate sig;

#[macro_use]
mod errors;
mod cargo_metadata;
mod child_process;
mod code_block;
mod command_context;
mod crash_guard;
mod crate_config;
mod eval_context;
#[allow(dead_code)]
mod evcxr_internal_runtime;
mod item;
mod module;
mod runtime;
mod rust_analyzer;
mod statement_splitter;
mod use_trees;

pub use crate::{
    command_context::CommandContext,
    errors::{CompilationError, JupyterError, JupyterErrorKind, JupyterResult, Theme},
    eval_context::{EvalCallbacks, EvalContext, EvalContextOutputs, EvalOutputs},
    runtime::runtime_hook,
};
pub use rust_analyzer::Completions;
pub use serde_json::{from_str as json_from_str, to_string_pretty as json_to_string, Value as JsonValue};
/// Return the directory that evcxr tools should use for their configuration.
///
/// By default this is the `evcxr` subdirectory of whatever `dirs::config_dir()`
/// returns, but it can be overridden by the `EVCXR_CONFIG_DIR` environment
/// variable.
pub fn config_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("EVCXR_CONFIG_DIR").map(std::path::PathBuf::from).or_else(|| dirs::config_dir().map(|d| d.join("evcxr")))
}
