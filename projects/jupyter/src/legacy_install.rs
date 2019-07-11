// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::{get_kernel_dir, JupyterResult, KernelConfig};
use serde_json::to_string_pretty;
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};

const LOGO_32X32: &[u8] = include_bytes!("../third_party/rust/rust-logo-32x32.png");
const LOGO_64X64: &[u8] = include_bytes!("../third_party/rust/rust-logo-64x64.png");
const LOGO_LICENSE: &[u8] = include_bytes!("../third_party/rust/LICENSE.md");
const KERNEL_JS: &[u8] = include_bytes!("../client/kernel.js");
const LINT_JS: &[u8] = include_bytes!("../third_party/CodeMirror/addons/lint/lint.js");
const LINT_CSS: &[u8] = include_bytes!("../third_party/CodeMirror/addons/lint/lint.css");
const LINT_LICENSE: &[u8] = include_bytes!("../third_party/CodeMirror/LICENSE");
const VERSION_TXT: &[u8] = include_bytes!("../client/version.txt");

pub(crate) fn install() -> JupyterResult<()> {
    let kernel_dir = get_kernel_dir()?;
    fs::create_dir_all(&kernel_dir)?;
    let kernel_config = KernelConfig::new("rust", "Rust")?;
    let kernel_json = to_string_pretty(&kernel_config)?;
    let kernel_json_filename = kernel_dir.join("kernel.json");
    println!("Writing {}", kernel_json_filename.to_string_lossy());
    // prerry print json
    let mut file = fs::File::create(&kernel_json_filename)?;
    file.write_all(kernel_json.as_bytes())?;

    install_resource(&kernel_dir, "logo-32x32.png", LOGO_32X32)?;
    install_resource(&kernel_dir, "logo-64x64.png", LOGO_64X64)?;
    install_resource(&kernel_dir, "logo-LICENSE.md", LOGO_LICENSE)?;
    install_resource(&kernel_dir, "kernel.js", KERNEL_JS)?;
    install_resource(&kernel_dir, "lint.js", LINT_JS)?;
    install_resource(&kernel_dir, "lint.css", LINT_CSS)?;
    install_resource(&kernel_dir, "lint-LICENSE", LINT_LICENSE)?;
    install_resource(&kernel_dir, "version.txt", VERSION_TXT)?;
    println!("Installation complete");
    Ok(())
}

/// Checks if the current installation is out-of-date, by looking at what's in
/// version.txt. If it is out of date, then updates it.
pub(crate) fn update_if_necessary() -> JupyterResult<()> {
    let kernel_dir = get_kernel_dir()?;
    // If the kernel directory doesn't exist, then we're probably being run from
    // a wrapper, so we shouldn't "update", since that would in effect be
    // installing ourselves when we weren't already installed.
    if !kernel_dir.exists() {
        return Ok(());
    }
    let installed_version = std::fs::read(kernel_dir.join("version.txt")).unwrap_or_default();
    if installed_version != VERSION_TXT {
        install()?;
        eprintln!(
            "\n\n==================================================================\n\
            Updated Evcxr Jupyter installation. Note, updates unfortunately \n\
            won't take effect until the next time you start jupyter notebook.\n\
            ==================================================================\n"
        );
    }
    Ok(())
}

pub(crate) fn install_resource(dir: &Path, filename: &str, bytes: &'static [u8]) -> JupyterResult<()> {
    let res_path = dir.join(filename);
    println!("Writing {}", res_path.to_string_lossy());
    let mut file = fs::File::create(res_path)?;
    file.write_all(bytes)?;
    Ok(())
}
