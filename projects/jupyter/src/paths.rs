use dirs::home_dir;
use std::env;
use std::path::PathBuf;

pub(crate) fn jupyter_runtime_dir() -> PathBuf {
    if let Ok(p) = env::var("JUPYTER_RUNTIME_DIR") {
        PathBuf::from(p)
    } else {
        os_jupyter_runtime_dir()
    }
}

pub(crate) fn jupyter_data_dir() -> PathBuf {
    if let Ok(p) = env::var("JUPYTER_DATA_DIR") {
        PathBuf::from(p)
    } else {
        os_jupyter_data_dir()
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn os_jupyter_runtime_dir() -> PathBuf {
    jupyter_data_dir().join("runtime")
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn os_jupyter_runtime_dir() -> PathBuf {
    if let Ok(p) = env::var("XDG_RUNTIME_DIR") {
        PathBuf::from(p).join("jupyter")
    } else {
        jupyter_data_dir().join("runtime")
    }
}

#[cfg(target_os = "macos")]
fn os_jupyter_data_dir() -> PathBuf {
    let home = home_dir().unwrap();
    home.join("Library").join("Jupyter")
}

#[cfg(target_os = "linux")]
fn os_jupyter_data_dir() -> PathBuf {
    if let Ok(p) = env::var("XDG_DATA_HOME") {
        PathBuf::from(p).join("jupyter")
    } else {
        let home = home_dir().unwrap();
        home.join(".local").join("share")
    }
}

#[cfg(target_os = "windows")]
fn os_jupyter_data_dir() -> PathBuf {
    if let Ok(app_data) = env::var("APPDATA") {
        PathBuf::from(app_data).join("jupyter")
    } else {
        unimplemented!()
    }
}
