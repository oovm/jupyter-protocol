use super::*;

#[derive(Debug, Clone)]
pub enum JupyterMessageType {
    /// https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-info
    KernelInfoRequest,
    Custom(String),
}

impl Default for JupyterMessageType {
    fn default() -> Self {
        JupyterMessageType::Custom("".to_string())
    }
}

impl FromStr for JupyterMessageType {
    type Err = JupyterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kernel_info_request" => Ok(JupyterMessageType::KernelInfoRequest),
            s => Ok(JupyterMessageType::Custom(s.to_string())),
        }
    }
}

impl AsRef<str> for JupyterMessageType {
    fn as_ref(&self) -> &str {
        match self {
            JupyterMessageType::KernelInfoRequest => "kernel_info_request",
            JupyterMessageType::Custom(v) => v,
        }
    }
}

impl Display for JupyterMessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl JupyterMessageType {
    pub fn is_empty(&self) -> bool {
        match self {
            JupyterMessageType::Custom(v) => v.is_empty(),
            _ => false,
        }
    }
}
