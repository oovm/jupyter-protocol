use super::*;

/// The sockets for Jupyter kernel.
pub struct JupyterKernelSockets {
    execution_result: Arc<Mutex<Option<UnboundedSender<ExecutionResult>>>>,
}

impl Debug for JupyterKernelSockets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let execution_channel = self.execution_result.try_lock().is_ok();
        f.debug_struct("JupyterKernelSockets").field("execution_channel", &execution_channel).finish()
    }
}

impl Default for JupyterKernelSockets {
    fn default() -> Self {
        Self { execution_result: Arc::new(Mutex::new(None)) }
    }
}

impl Clone for JupyterKernelSockets {
    fn clone(&self) -> Self {
        Self { execution_result: self.execution_result.clone() }
    }
}

impl JupyterKernelSockets {
    /// Create a new [JupyterKernelSockets].
    pub async fn bind_execution_socket(&self, sender: UnboundedSender<ExecutionResult>) {
        let mut channel = self.execution_result.lock().await;
        *channel = Some(sender);
    }
    /// Send an executed result.
    pub async fn send_executed(&self, executed: impl Executed) {
        match self.try_send_executed(executed).await {
            Ok(_) => (),
            Err(_) => (),
        }
    }
    async fn try_send_executed(&self, executed: impl Executed) -> JupyterResult<()> {
        let data = ExecutionResult::default().with_data(executed.mime_type(), executed.as_json(&JupyterContext::default()));
        self.send_executed_result(data).await
    }
    /// Send an execution result.
    pub async fn send_executed_result(&self, result: ExecutionResult) -> JupyterResult<()> {
        let mut channel = self.execution_result.lock().await;
        match channel.as_mut() {
            Some(sender) => Ok(sender.send(result)?),
            None => Err(JupyterError::channel_block("ExecutionResult")),
        }
    }
}
