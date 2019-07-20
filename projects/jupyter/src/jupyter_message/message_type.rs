use super::*;

#[derive(Debug, Clone)]
pub enum JupyterMessageType {
    /// - [status](https://jupyter-client.readthedocs.io/en/stable/messaging.html#status)
    StatusReply,
    /// - [comm_info_request](https://jupyter-client.readthedocs.io/en/stable/messaging.html#comm-info)
    CommonInfoRequest,
    /// - [comm_info_reply](https://jupyter-client.readthedocs.io/en/stable/messaging.html#comm-info)
    CommonInfoReply,
    /// - [execute_request](https://jupyter-client.readthedocs.io/en/stable/messaging.html#code-inputs)
    ExecuteRequest,
    /// - [execute_result](https://jupyter-client.readthedocs.io/en/stable/messaging.html#execution-results)
    ExecuteReply,
    /// - [kernel_info_request](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-info)
    KernelInfoRequest,
    /// - [kernel_info_reply](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-info)
    KernelInfoReply,
    /// - [custom](https://jupyter-client.readthedocs.io/en/stable/messaging.html#custom-messages)
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
            JupyterMessageType::StatusReply => "status",
            JupyterMessageType::KernelInfoRequest => "kernel_info_request",
            JupyterMessageType::KernelInfoReply => "kernel_info_reply",
            JupyterMessageType::CommonInfoRequest => "comm_info_request",
            JupyterMessageType::CommonInfoReply => "comm_info_reply",
            JupyterMessageType::ExecuteRequest => "execute_request",
            JupyterMessageType::ExecuteReply => "execute_result",
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
            "status" => JupyterMessageType::StatusReply,
            "kernel_info" | "kernel_info_request" => JupyterMessageType::KernelInfoRequest,
            "kernel_info_reply" => JupyterMessageType::KernelInfoReply,
            "comm_info_request" => JupyterMessageType::CommonInfoRequest,
            "comm_info_reply" => JupyterMessageType::CommonInfoReply,
            "execute_request" => JupyterMessageType::ExecuteRequest,
            "execute_reply" => JupyterMessageType::ExecuteReply,
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
            JupyterMessageType::CommonInfoRequest => JupyterMessageType::CommonInfoReply,
            JupyterMessageType::ExecuteRequest => JupyterMessageType::ExecuteReply,
            JupyterMessageType::Custom(s) => JupyterMessageType::Custom(s.replace("_request", "_reply")),
            _ => self.clone(),
        }
    }
}
