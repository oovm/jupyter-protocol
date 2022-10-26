// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::{get_kernel_dir, JupyterResult, JupyterServerProtocol, KernelConfig};
use serde_json::to_string_pretty;
use std::{io::Write, path::Path};

const KERNEL_JS: &[u8] = include_bytes!("../client/kernel.js");
const LINT_JS: &[u8] = include_bytes!("../third_party/CodeMirror/addons/lint/lint.js");
const LINT_CSS: &[u8] = include_bytes!("../third_party/CodeMirror/addons/lint/lint.css");
const LINT_LICENSE: &[u8] = include_bytes!("../third_party/CodeMirror/LICENSE");

pub(crate) fn install<T: JupyterServerProtocol>(info: &T) -> JupyterResult<()> {
    let info = info.language_info();
    let kernel_dir = get_kernel_dir(&info.language_key)?;
    std::fs::create_dir_all(&kernel_dir)?;
    let kernel_config = KernelConfig::new(&info.language_key, &info.language)?;
    let kernel_json = to_string_pretty(&kernel_config)?;
    let kernel_json_filename = kernel_dir.join("kernel.json");
    tracing::info!("Writing {}", kernel_json_filename.to_string_lossy());
    // prerry print json
    let mut file = std::fs::File::create(&kernel_json_filename)?;
    file.write_all(kernel_json.as_bytes())?;

    install_resource(&kernel_dir, "logo-32x32.png", info.png_32)?;
    install_resource(&kernel_dir, "logo-64x64.png", info.png_64)?;
    install_resource(&kernel_dir, "kernel.js", KERNEL_JS)?;
    install_resource(&kernel_dir, "lint.js", LINT_JS)?;
    install_resource(&kernel_dir, "lint.css", LINT_CSS)?;
    install_resource(&kernel_dir, "lint-LICENSE", LINT_LICENSE)?;
    // install_resource(&kernel_dir, "version.txt", VERSION_TXT)?;
    tracing::info!("Installation complete");
    Ok(())
}
pub(crate) fn install_resource(dir: &Path, filename: &str, bytes: &'static [u8]) -> JupyterResult<()> {
    let res_path = dir.join(filename);
    tracing::info!("Writing {}", res_path.to_string_lossy());
    let mut file = std::fs::File::create(res_path)?;
    file.write_all(bytes)?;
    Ok(())
}
