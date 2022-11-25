use super::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum JupyterMessageType {
    /// - [status](https://jupyter-client.readthedocs.io/en/stable/messaging.html#status)
    StatusReply,
    /// ???
    Stream,
    /// - [comm_info_request](https://jupyter-client.readthedocs.io/en/stable/messaging.html#comm-info)
    CommonInfoRequest,
    /// - [comm_info_reply](https://jupyter-client.readthedocs.io/en/stable/messaging.html#comm-info)
    CommonInfoReply,
    /// - [kernel_info_request](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-info)
    KernelInfoRequest,
    /// - [kernel_info_reply](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-info)
    KernelInfoReply,
    /// - [execute_request](https://jupyter-client.readthedocs.io/en/stable/messaging.html#code-inputs)
    ExecuteRequest,
    /// - [execute_result](https://jupyter-client.readthedocs.io/en/stable/messaging.html#execution-results)
    ExecuteResult,
    /// - [execute_result](https://jupyter-client.readthedocs.io/en/stable/messaging.html#execution-results)
    ExecuteReply,
    /// - [debug_request](https://jupyter-client.readthedocs.io/en/stable/messaging.html#debug-request)
    DebugRequest,
    /// - [debug_reply](https://jupyter-client.readthedocs.io/en/stable/messaging.html#debug-request)
    DebugReply,
    /// - [debug_event](https://jupyter-client.readthedocs.io/en/stable/messaging.html#debug-event)
    DebugEvent,
    /// - [interrupt_request](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-interrupt)
    InterruptRequest,
    /// - [interrupt_reply](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-interrupt)
    InterruptReply,
    /// - [shutdown_request](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-shutdown)
    ShutdownRequest,
    /// - [shutdown_reply](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-shutdown)
    ShutdownReply,
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
            Self::StatusReply => "status",
            Self::Stream => "stream",
            Self::KernelInfoRequest => "kernel_info_request",
            Self::KernelInfoReply => "kernel_info_reply",
            Self::CommonInfoRequest => "comm_info_request",
            Self::CommonInfoReply => "comm_info_reply",
            Self::ExecuteRequest => "execute_request",
            Self::ExecuteResult => "execute_result",
            Self::ExecuteReply => "execute_reply",
            Self::DebugRequest => "debug_request",
            Self::DebugReply => "debug_reply",
            Self::DebugEvent => "debug_event",
            Self::InterruptRequest => "interrupt_request",
            Self::InterruptReply => "interrupt_reply",
            Self::ShutdownRequest => "shutdown_request",
            Self::ShutdownReply => "shutdown_reply",
            Self::Custom(v) => v,
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
            "comm_info_request" => JupyterMessageType::CommonInfoRequest,
            "execute_request" => JupyterMessageType::ExecuteRequest,
            "debug_request" => JupyterMessageType::DebugRequest,
            "interrupt_request" => JupyterMessageType::InterruptRequest,
            "shutdown_request" => JupyterMessageType::ShutdownRequest,
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
            JupyterMessageType::InterruptRequest => JupyterMessageType::InterruptReply,
            JupyterMessageType::ShutdownRequest => JupyterMessageType::ShutdownReply,
            JupyterMessageType::DebugRequest => JupyterMessageType::DebugReply,
            JupyterMessageType::Custom(s) => JupyterMessageType::Custom(s.replace("_request", "_reply")),
            _ => self.clone(),
        }
    }
}
