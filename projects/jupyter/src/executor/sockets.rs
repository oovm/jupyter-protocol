use crate::{Executed, ExecutionResult, JupyterError, JupyterResult, JupyterTheme};
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedSender, Mutex};

pub struct JupyterServerSockets {
    execution_result: Arc<Mutex<Option<UnboundedSender<ExecutionResult>>>>,
}

impl Default for JupyterServerSockets {
    fn default() -> Self {
        Self { execution_result: Arc::new(Mutex::new(None)) }
    }
}

impl Clone for JupyterServerSockets {
    fn clone(&self) -> Self {
        Self { execution_result: self.execution_result.clone() }
    }
}

impl JupyterServerSockets {
    pub async fn bind_execution_socket(&self, sender: UnboundedSender<ExecutionResult>) {
        let mut channel = self.execution_result.lock().await;
        *channel = Some(sender);
    }
    pub async fn send_executed(&self, executed: impl Executed) {
        match self.try_send_executed(executed).await {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    async fn try_send_executed(&self, executed: impl Executed) -> JupyterResult<()> {
        let data = ExecutionResult::default().with_data(executed.mime_type(), executed.as_json(JupyterTheme::Light));
        self.send_executed_result(data).await
    }
    pub async fn send_executed_result(&self, result: ExecutionResult) -> JupyterResult<()> {
        let mut channel = self.execution_result.lock().await;
        match channel.as_mut() {
            Some(sender) => Ok(sender.send(result)?),
            None => Err(JupyterError::channel_block("ExecutionResult")),
        }
    }
}
