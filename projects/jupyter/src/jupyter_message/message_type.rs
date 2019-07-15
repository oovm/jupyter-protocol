use super::*;

#[derive(Debug, Clone)]
pub enum JupyterMessageType {
    Status,
    /// https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-info
    KernelInfoRequest,
    KernelInfoReply,
    Custom(String),
}

impl Default for JupyterMessageType {
    fn default() -> Self {
        JupyterMessageType::Custom("".to_string())
    }
}

impl FromStr for JupyterMessageType {
    type Err = JupyterError;

    fn from_str(s: &str) -> JupyterResult<Self> {
        Ok(JupyterMessageType::new(s))
    }
}

impl AsRef<str> for JupyterMessageType {
    fn as_ref(&self) -> &str {
        match self {
            JupyterMessageType::Status => "status",
            JupyterMessageType::KernelInfoRequest => "kernel_info_request",
            JupyterMessageType::KernelInfoReply => "kernel_info_reply",
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
    pub fn new(kind: &str) -> JupyterMessageType {
        match kind {
            "status" => JupyterMessageType::Status,
            "kernel_info_request" => JupyterMessageType::KernelInfoRequest,
            s => JupyterMessageType::Custom(s.to_string()),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            JupyterMessageType::Custom(v) => v.is_empty(),
            _ => false,
        }
    }
    pub fn as_reply(&self) -> Self {
        match self {
            JupyterMessageType::KernelInfoRequest => JupyterMessageType::KernelInfoReply,
            JupyterMessageType::Custom(s) => JupyterMessageType::Custom(s.replace("_request", "_reply")),
            _ => self.clone(),
        }
    }
}
