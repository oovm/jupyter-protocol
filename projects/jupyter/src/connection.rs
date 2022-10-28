// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::errors::JupyterResult;
use hmac::{digest::KeyInit, Hmac};
use sha2::Sha256;

pub(crate) const KERNEL_JS: &[u8] = include_bytes!("../client/kernel.js");
pub(crate) const LINT_JS: &[u8] = include_bytes!("../third_party/CodeMirror/addons/lint/lint.js");
pub(crate) const LINT_CSS: &[u8] = include_bytes!("../third_party/CodeMirror/addons/lint/lint.css");
pub(crate) const LINT_LICENSE: &[u8] = include_bytes!("../third_party/CodeMirror/LICENSE");

pub(crate) type HmacSha256 = Hmac<Sha256>;

pub(crate) struct Connection<S> {
    pub(crate) socket: S,
    /// Will be None if our key was empty (digest authentication disabled).
    pub(crate) mac: Option<HmacSha256>,
}

impl<S: zeromq::Socket> Connection<S> {
    pub(crate) fn new(socket: S, key: &str) -> JupyterResult<Self> {
        let mac = if key.is_empty() {
            None
        }
        else {
            Some(HmacSha256::new_from_slice(key.as_bytes()).expect("Shouldn't fail with HMAC"))
        };
        Ok(Connection { socket, mac })
    }
}
