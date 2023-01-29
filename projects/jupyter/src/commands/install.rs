use super::*;
use serde_json::to_string_pretty;

/// To install/overwrite a new kernel to jupyter.
#[derive(Clone, Debug, Parser)]
pub struct InstallAction {
    /// Optional name to operate on
    name: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
struct KernelConfig {
    argv: Vec<String>,
    display_name: String,
    language: String,
    interrupt_mode: String,
    metadata: Metadata,
}

#[derive(Clone, Debug, Serialize)]
struct Metadata {
    debugger: bool,
}

impl InstallAction {
    /// Run the install action.
    pub fn run<T>(&self, engine: T) -> JupyterResult<()>
    where
        T: JupyterKernelProtocol,
    {
        do_install(&engine)
    }
}

impl KernelConfig {
    pub fn new(language: &str, display: &str) -> JupyterResult<Self> {
        match std::env::current_exe() {
            Ok(path) => Ok(Self {
                argv: vec![
                    path.to_string_lossy().to_string(),
                    "start".to_string(),
                    "--control-file".to_string(),
                    "{connection_file}".to_string(),
                ],
                display_name: display.to_string(),
                language: language.to_string(),
                interrupt_mode: "message".to_string(),
                metadata: Metadata { debugger: true },
            }),
            Err(e) => {
                // "current exe path isn't valid UTF-8"
                panic!("Couldn't get current exe path: {}", e);
            }
        }
    }
}

pub(crate) fn do_install<T: JupyterKernelProtocol>(info: &T) -> JupyterResult<()> {
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
