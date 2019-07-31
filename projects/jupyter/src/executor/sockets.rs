use crate::{Executed, ExecutionResult};
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
    pub async fn bind_execution_result(&self, sender: UnboundedSender<ExecutionResult>) {
        let channel = self.execution_result.lock().await;
        *channel = Some(sender);
    }
    pub async fn send_executed(&self, executed: impl Executed) {
        executed.as_json()
    }
    pub async fn send_execution_result(&self, result: ExecutionResult) {
        let channel = self.execution_result.lock().await;
        if let Some(channel) = &*channel {
            match channel.send(result) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error sending execution result: {}", e);
                }
            }
        }
    }
}
