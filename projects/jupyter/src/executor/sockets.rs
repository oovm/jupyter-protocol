use super::*;
use crate::{
    connection::Connection,
    jupyter_message::{JupyterMessage, JupyterMessageHeader, JupyterMessageType},
};
use serde::Serialize;
use std::path::PathBuf;
use zeromq::PubSocket;

/// Indicates successful establishment of link with jupyter frontend
#[derive(Debug)]
pub struct JupyterConnection {
    /// startup path of jupyter
    pub boot_path: PathBuf,
    /// sockets for returning execution results
    pub sockets: JupyterKernelSockets,
}

/// The sockets for Jupyter kernel.
pub struct JupyterKernelSockets {
    // pub(crate) execute_channel: Option<Arc<Mutex<UnboundedSender<ExecutionResult>>>>,
    pub(crate) io_channel: Option<Arc<Mutex<Connection<PubSocket>>>>,
}

impl Default for JupyterKernelSockets {
    fn default() -> Self {
        Self { io_channel: None }
    }
}

impl Debug for JupyterKernelSockets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let io_channel = match &self.io_channel {
            Some(c) => c.try_lock().is_ok(),
            None => false,
        };
        f.debug_struct("JupyterKernelSockets").field("ui_channel", &io_channel).finish()
    }
}
impl Clone for JupyterKernelSockets {
    fn clone(&self) -> Self {
        Self { io_channel: self.io_channel.clone() }
    }
}

#[derive(Debug, Serialize)]
pub struct JupyterStream {
    name: &'static str,
    text: String,
}

impl JupyterStream {
    pub fn std_out<S: ToString>(text: S) -> Self {
        JupyterStream { name: "stdout", text: text.to_string() }
    }
}

impl JupyterKernelSockets {
    /// Send an executed result.
    pub async fn send_executed(&self, executed: impl Executed, count: u32, parent: &JupyterMessage) {
        self.try_send_executed(executed, count, parent).await.ok();
    }
    /// Send information through io stream, such as `print`
    pub async fn send_stream(&self, stream: JupyterStream, parent: &JupyterMessage) {
        self.try_send_io_stream(stream, parent).await.ok();
    }

    async fn try_send_executed(&self, executed: impl Executed, count: u32, parent: &JupyterMessage) -> JupyterResult<()> {
        let data = ExecutionResult::default().with_data(executed.mime_type(), executed.as_json(&JupyterContext::default()));
        match &self.io_channel {
            Some(channel) => {
                parent
                    .as_reply()
                    .with_content(data.with_count(count))?
                    .with_message_type(JupyterMessageType::ExecuteResult)
                    .send_by(&mut &mut channel.lock().await)
                    .await
            }
            None => Err(JupyterError::custom("Missing execute channel")),
        }
    }

    async fn try_send_io_stream(&self, stream: JupyterStream, parent: &JupyterMessage) -> JupyterResult<()> {
        match &self.io_channel {
            Some(channel) => {
                parent
                    .as_reply()
                    .with_content(stream)?
                    .with_message_type(JupyterMessageType::Stream)
                    .send_by(&mut &mut channel.lock().await)
                    .await
            }
            None => Err(JupyterError::custom("Missing IO channel")),
        }
    }
}
