// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::{
    connection::Connection,
    errors::JupyterResult,
    jupyter_message::{JupiterContent, JupyterMessage, JupyterMessageType},
    ExecutionResult, ExecutionState, JupyterServerProtocol, KernelControl, SinkExecutor,
};
use std::collections::HashMap;

use crate::executor::Executed;
use serde_json::{json, Value};
use std::{
    sync::Arc,
    time::{Duration, SystemTime, SystemTimeError},
};
use tokio::{
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        Mutex,
    },
    task::{JoinError, JoinHandle},
};
use zeromq::{PubSocket, RepSocket, RouterSocket, Socket, SocketRecv, SocketSend, ZmqMessage};

// Note, to avoid potential deadlocks, each thread should lock at most one mutex at a time.
#[derive(Clone)]
pub(crate) struct Server {
    heartbeat: Arc<Mutex<Connection<RepSocket>>>,
    iopub: Arc<Mutex<Connection<PubSocket>>>,
    stdin: Arc<Mutex<Connection<RouterSocket>>>,
    control: Arc<Mutex<Connection<RouterSocket>>>,
    shell_socket: Arc<Mutex<Connection<RouterSocket>>>,
    latest_execution_request: Arc<Mutex<Option<JupyterMessage>>>,
    execution_request_receiver: Arc<Mutex<UnboundedReceiver<JupyterMessage>>>,
    shutdown_sender: Arc<Mutex<Option<crossbeam_channel::Sender<()>>>>,
    tokio_handle: tokio::runtime::Handle,
}

pub struct ExecuteProvider<T> {
    pub context: Arc<Mutex<T>>,
}

impl<T> Clone for ExecuteProvider<T> {
    fn clone(&self) -> Self {
        Self { context: self.context.clone() }
    }
}

impl<T> ExecuteProvider<T> {
    pub fn new(context: T) -> Self
    where
        T: JupyterServerProtocol + 'static,
    {
        Self { context: Arc::new(Mutex::new(context)) }
    }
}

struct ShutdownReceiver {
    // Note, this needs to be a crossbeam channel because
    // start_output_pass_through_thread selects on this and other crossbeam
    // channels.
    recv: crossbeam_channel::Receiver<()>,
}

impl Server {
    pub(crate) fn run(config: &KernelControl) -> JupyterResult<()> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            // We only technically need 1 thread. However we've observed that
            // when using vscode's jupyter extension, we can get requests on the
            // shell socket before we have any subscribers on iopub. The iopub
            // subscription then completes, but the execution_state="idle"
            // message(s) have already been sent to a channel that at the time
            // had no subscriptions. The vscode extension then waits
            // indefinitely for an execution_state="idle" message that will
            // never come. Having multiple threads at least reduces the chances
            // of this happening.
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap();
        let handle = runtime.handle().clone();
        runtime.block_on(async {
            let shutdown_receiver = Self::start(config, handle).await?;
            shutdown_receiver.wait_for_shutdown().await;
            let result: JupyterResult<()> = Ok(());
            result
        })?;
        Ok(())
    }

    async fn start(config: &KernelControl, tokio_handle: tokio::runtime::Handle) -> JupyterResult<ShutdownReceiver> {
        let heartbeat = bind_socket::<RepSocket>(config, config.hb_port).await?;
        let shell_socket = bind_socket::<RouterSocket>(config, config.shell_port).await?;
        let control_socket = bind_socket::<RouterSocket>(config, config.control_port).await?;
        let stdin_socket = bind_socket::<RouterSocket>(config, config.stdin_port).await?;
        let iopub_socket = bind_socket::<PubSocket>(config, config.iopub_port).await?;
        let iopub = Arc::new(Mutex::new(iopub_socket));
        let (shutdown_sender, shutdown_receiver) = crossbeam_channel::unbounded();
        let (execution_result_sender, execution_receiver) = tokio::sync::mpsc::unbounded_channel();
        // let (execution_res ponse_sender, mut execution_response_receiver) = tokio::sync::mpsc::unbounded_channel();
        let server = Server {
            iopub,
            heartbeat: Arc::new(Mutex::new(heartbeat)),
            latest_execution_request: Arc::new(Mutex::new(None)),
            execution_request_sender: Arc::new(Mutex::new(execution_result_sender)),
            execution_request_receiver: Arc::new(Mutex::new(execution_receiver)),
            stdin: Arc::new(Mutex::new(stdin_socket)),
            control: Arc::new(Mutex::new(control_socket)),
            shutdown_sender: Arc::new(Mutex::new(Some(shutdown_sender))),
            tokio_handle,
            shell_socket: Arc::new(Mutex::new(shell_socket)),
        };
        let context = ExecuteProvider::new(SinkExecutor::default());
        server.clone().spawn_heart_beat();
        server.clone().spawn_shell_execution(context.clone());
        // server.clone().spawn_execution_queue(context.clone());
        // server.clone().spawn_control();
        Ok(ShutdownReceiver { recv: shutdown_receiver })
    }

    async fn signal_shutdown(&mut self) {
        self.shutdown_sender.lock().await.take();
    }
    fn spawn_heart_beat(self) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                if let Err(e) = self.clone().handle_heart_beat().await {
                    eprintln!("Error sending heartbeat: {:?}", e);
                }
            }
        })
    }
    async fn handle_heart_beat(self) -> JupyterResult<()> {
        let mut connection = match self.heartbeat.try_lock() {
            Ok(o) => o,
            Err(_) => return Ok(()),
        };
        let _ = connection.socket.recv().await?;
        connection.socket.send(ZmqMessage::from(b"ping".to_vec())).await?;
        Ok(())
    }
    fn spawn_shell_execution<T>(self, executor: ExecuteProvider<T>) -> JoinHandle<()>
    where
        T: JupyterServerProtocol + Send + 'static,
    {
        let mut count = 0;
        tokio::spawn(async move {
            println!("Shell Executor Spawned");
            loop {
                if let Err(e) = self.clone().handle_shell(executor.clone(), &mut count).await {
                    eprintln!("Error sending shell execution: {:?}", e);
                }
            }
        })
    }
    async fn handle_shell<'a, T>(self, executor: ExecuteProvider<T>, count: &mut u32) -> JupyterResult<()>
    where
        T: JupyterServerProtocol + Send + 'static,
    {
        // Processing of every message should be enclosed between "busy" and "idle"
        // see https://jupyter-client.readthedocs.io/en/latest/messaging.html#messages-on-the-shell-router-dealer-channel
        // Jupiter Lab doesn't use the kernel until it received "idle" for kernel_info_request
        let io = &mut self.iopub.lock().await;
        let mut shell = &mut self.shell_socket.lock().await;
        let request = JupyterMessage::read(&mut shell).await?;
        let busy = request.create_message(JupyterMessageType::StatusReply).with_content(ExecutionState::new("busy"));
        let idle = request.create_message(JupyterMessageType::StatusReply).with_content(ExecutionState::new("idle"));
        busy.send(io).await?;
        match request.kind() {
            JupyterMessageType::KernelInfoRequest => {
                let info = executor.context.lock().await.language_info();
                let cont = JupiterContent::build_kernel_info(info);
                request.as_reply().with_content(cont).send(shell).await?
            }
            JupyterMessageType::ExecuteRequest => {
                let time = SystemTime::now();
                *count += 1;
                let mut task = request.as_execution_request()?;
                task.execution_count = *count;
                // reply busy event
                let mut runner = executor.context.lock().await;
                for result in runner.running(task.clone()).await {
                    let any = task.as_result(result.mime_type(), result.as_json(), *count)?;
                    request.as_reply().with_message_type(JupyterMessageType::ExecuteResult).with_content(any).send(io).await?;
                }
                // Check elapsed time
                match time.elapsed() {
                    Ok(o) => {
                        let escape = runner.running_time(o.as_secs_f64());
                        if !escape.is_empty() {
                            let time = task.as_result("text/html", escape, *count)?;
                            request
                                .as_reply()
                                .with_message_type(JupyterMessageType::ExecuteResult)
                                .with_content(time)
                                .send(io)
                                .await?;
                        }
                    }
                    Err(_) => {}
                }
                // reply finish event
                let reply = request.as_execution_request()?.as_reply(true, *count)?;
                request.as_reply().with_content(reply).send(shell).await?;
            }
            JupyterMessageType::CommonInfoRequest => {
                println!("Unsupported message: {:?}", request.kind());
            }
            JupyterMessageType::Custom(v) => {
                println!("Got custom shell message: {:?}", v);
            }
            _ => {
                println!("Got unknown shell message: {:?}", request);
            }
        }
        idle.send(io).await?;
        Ok(())
    }

    fn spawn_execution_queue<T>(self, executor: ExecuteProvider<T>) -> JoinHandle<()>
    where
        T: JupyterServerProtocol + Send + 'static,
    {
        let mut running_count = 0;
        tokio::spawn(async move {
            println!("Queue Executor Spawned");
            loop {
                if let Err(e) = self.clone().handle_execution_queue(executor.clone(), running_count).await {
                    eprintln!("Error sending execution queue: {:?}", e);
                }
                running_count += 1;
            }
        })
    }
    async fn handle_execution_queue<T>(self, _executor: ExecuteProvider<T>, _count: i32) -> JupyterResult<()>
    where
        T: JupyterServerProtocol + Send + 'static,
    {
        todo!();
        // let zmq = match self.execution_request_receiver.lock().await.recv().await {
        //     Some(s) => s,
        //     None => return Ok(()),
        // };
        // println!("Waiting for execution request {}: {:?}", count, zmq);
        // let io = &mut self.shell_socket.try_lock()?;
        // println!("ok1");
        // let result = zmq.as_execution_request()?.as_reply(2, 1)?;
        // println!("ok2");
        // let reply = zmq.as_reply().with_content(result);
        // println!("ok3");
        // reply.send(io).await?;
        // println!("Finished execution request {}: {:?}", count, reply);
        // Ok(())
    }

    async fn spawn_control(self) -> Result<(), JoinError> {
        let task = tokio::spawn(async move {
            loop {
                match self.control.try_lock() {
                    Ok(mut o) => match o.socket.recv().await {
                        Ok(msg) => {
                            println!("Got control message: {:?}", msg);
                            o.socket.send(msg).await.unwrap();
                        }
                        Err(e) => {
                            eprintln!("Error receiving control message: {:?}", e);
                            break;
                        }
                    },
                    Err(_) => continue,
                };
            }
        });
        task.await
    }
}

impl ShutdownReceiver {
    async fn wait_for_shutdown(self) {
        let _ = tokio::task::spawn_blocking(move || self.recv.recv()).await;
    }
}

async fn bind_socket<S: Socket>(config: &KernelControl, port: u16) -> JupyterResult<Connection<S>> {
    let endpoint = format!("{}://{}:{}", config.transport, config.ip, port);
    let mut socket = S::new();
    socket.bind(&endpoint).await?;
    Connection::new(socket, &config.key)
}

async fn handle_completion_request<T>(
    _context: &Arc<std::sync::Mutex<ExecuteProvider<T>>>,
    _message: JupyterMessage,
) -> JupyterResult<Value> {
    // let context = Arc::clone(context);
    // tokio::task::spawn_blocking(move || {
    //     println!("{message:#?}");
    //     panic!()
    // })
    // .await?
    todo!()
}

/// Returns the byte offset for the start of the specified grapheme. Any grapheme beyond the last
/// grapheme will return the end position of the input.
fn grapheme_offset_to_byte_offset(code: &str, grapheme_offset: usize) -> usize {
    unicode_segmentation::UnicodeSegmentation::grapheme_indices(code, true)
        .nth(grapheme_offset)
        .map(|(byte_offset, _)| byte_offset)
        .unwrap_or_else(|| code.len())
}

/// Returns the grapheme offset of the grapheme that starts at
fn byte_offset_to_grapheme_offset(code: &str, target_byte_offset: usize) -> JupyterResult<usize> {
    let mut grapheme_offset = 0;
    for (byte_offset, _) in unicode_segmentation::UnicodeSegmentation::grapheme_indices(code, true) {
        if byte_offset == target_byte_offset {
            break;
        }
        if byte_offset > target_byte_offset {
            panic!("Byte offset {} is not on a grapheme boundary in '{}'", target_byte_offset, code);
        }
        grapheme_offset += 1;
    }
    Ok(grapheme_offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grapheme_offsets() {
        let src = "a̐éx";
        assert_eq!(grapheme_offset_to_byte_offset(src, 0), 0);
        assert_eq!(grapheme_offset_to_byte_offset(src, 1), 3);
        assert_eq!(grapheme_offset_to_byte_offset(src, 2), 6);
        assert_eq!(grapheme_offset_to_byte_offset(src, 3), 7);

        assert_eq!(byte_offset_to_grapheme_offset(src, 0).unwrap(), 0);
        assert_eq!(byte_offset_to_grapheme_offset(src, 3).unwrap(), 1);
        assert_eq!(byte_offset_to_grapheme_offset(src, 6).unwrap(), 2);
        assert_eq!(byte_offset_to_grapheme_offset(src, 7).unwrap(), 3);
    }
}
