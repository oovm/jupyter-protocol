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
    ExecutionState, KernelControl,
};
use ariadne::sources;
use bytes::Bytes;
use colored::*;
use crossbeam_channel::Select;
use serde_json::Value;
use std::{
    collections::HashMap,
    future::Future,
    ops::{Deref, DerefMut},
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{
        mpsc::{error::SendError, UnboundedReceiver, UnboundedSender},
        Mutex, MutexGuard, TryLockError,
    },
    task::{JoinError, JoinHandle},
};
use zeromq::{PubSocket, RepSocket, RouterSocket, Socket, SocketRecv, SocketSend, ZmqMessage, ZmqResult};

// Note, to avoid potential deadlocks, each thread should lock at most one mutex at a time.
#[derive(Clone)]
pub(crate) struct Server {
    heartbeat: Arc<Mutex<Connection<RepSocket>>>,
    iopub: Arc<Mutex<Connection<PubSocket>>>,
    stdin: Arc<Mutex<Connection<RouterSocket>>>,
    control: Arc<Mutex<Connection<RouterSocket>>>,
    shell_socket: Arc<Mutex<Connection<RouterSocket>>>,
    latest_execution_request: Arc<Mutex<Option<JupyterMessage>>>,
    shutdown_sender: Arc<Mutex<Option<crossbeam_channel::Sender<()>>>>,
    tokio_handle: tokio::runtime::Handle,
}

pub struct ExecuteProvider<T> {
    context: Arc<Mutex<T>>,
}

impl<T> Clone for ExecuteProvider<T> {
    fn clone(&self) -> Self {
        Self { context: self.context.clone() }
    }
}

pub trait ExecuteContext {
    fn language_info(&self) -> LanguageInfo;
}

pub struct LanguageInfo {
    pub language: String,
    pub file_extensions: String,
}

pub struct SinkExecutor {
    name: String,
}

impl ExecuteContext for SinkExecutor {
    fn language_info(&self) -> LanguageInfo {
        LanguageInfo { language: "Rust".to_string(), file_extensions: ".rs".to_string() }
    }
}

impl<T> ExecuteProvider<T> {
    pub fn new(context: T) -> Self
    where
        T: ExecuteContext + 'static,
    {
        Self { context: Arc::new(Mutex::new(context)) }
    }

    pub(crate) fn execute(&self, p0: &str) -> JupyterResult<()> {
        println!("execute: {}", p0);
        Ok(())
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
        let mut heartbeat = bind_socket::<RepSocket>(config, config.hb_port).await?;
        let shell_socket = bind_socket::<RouterSocket>(config, config.shell_port).await?;
        let control_socket = bind_socket::<RouterSocket>(config, config.control_port).await?;
        let stdin_socket = bind_socket::<RouterSocket>(config, config.stdin_port).await?;
        let iopub_socket = bind_socket::<PubSocket>(config, config.iopub_port).await?;
        let iopub = Arc::new(Mutex::new(iopub_socket));

        let (shutdown_sender, shutdown_receiver) = crossbeam_channel::unbounded();

        let server = Server {
            iopub,
            heartbeat: Arc::new(Mutex::new(heartbeat)),
            latest_execution_request: Arc::new(Mutex::new(None)),
            stdin: Arc::new(Mutex::new(stdin_socket)),
            control: Arc::new(Mutex::new(control_socket)),
            shutdown_sender: Arc::new(Mutex::new(Some(shutdown_sender))),
            tokio_handle,
            shell_socket: Arc::new(Mutex::new(shell_socket)),
        };
        let mut context = ExecuteProvider::new(SinkExecutor { name: "sink".to_string() });
        let (execution_sender, mut execution_receiver) = tokio::sync::mpsc::unbounded_channel();
        let (execution_response_sender, mut execution_response_receiver) = tokio::sync::mpsc::unbounded_channel();
        server.clone().spawn_shell_executor(context.clone()).await.expect("spawn_heart_beat");

        {
            tokio::spawn(async move {
                loop {
                    match execution_response_receiver.recv().await {
                        Some(message) => {
                            println!("execution_response_receiver.recv: {:?}", message);
                        }
                        None => {
                            println!("execution_response_receiver.recv: None");
                            break;
                        }
                    }
                }
            });
        }

        // context.execute(":load_config")?;
        // let process_handle = context.process_handle();
        // let context = Arc::new(std::sync::Mutex::new(context));
        if let Err(e) = server.clone().spawn_control().await {
            eprintln!("Spawn control fail: {:?}", e);
        }
        server.clone().spawn_heart_beat().await.expect("spawn shell executor failed");
        // {
        //     let context = context.clone();
        //     let server = server.clone();
        //     tokio::spawn(async move {
        //         let result =
        //             server.handle_shell(shell_socket, &execution_sender, &mut execution_response_receiver, context).await;
        //         if let Err(error) = result {
        //             eprintln!("shell error: {error:?}");
        //         }
        //     });
        // }
        {
            let server = server.clone();
            let context = context.clone();
            tokio::spawn(async move {
                let result =
                    server.handle_execution_requests(context, &mut execution_receiver, &execution_response_sender).await;
                if let Err(error) = result {
                    eprintln!("execution error: {error:?}");
                }
            });
        }
        // server
        //     .clone()
        //     .start_output_pass_through_thread(
        //         vec![("stdout", outputs.stdout), ("stderr", outputs.stderr)],
        //         shutdown_receiver.clone(),
        //     )
        //     .await;
        Ok(ShutdownReceiver { recv: shutdown_receiver })
    }

    async fn signal_shutdown(&mut self) {
        self.shutdown_sender.lock().await.take();
    }
    async fn spawn_shell_executor<T>(self, executor: ExecuteProvider<T>) -> Result<(), JoinError>
    where
        T: ExecuteContext + Send + 'static,
    {
        let task = tokio::spawn(async move {
            loop {
                let socket = self.shell_socket.lock().await;
                let io = self.iopub.lock().await;
                let exe = executor.context.lock().await;
                if let Err(e) = handle_shell(exe, io, socket).await {
                    eprintln!("Error sending heartbeat: {:?}", e);
                }
            }
        });
        task.await
    }
    async fn spawn_heart_beat(self) -> Result<(), JoinError> {
        let task = tokio::spawn(async move {
            loop {
                let o = self.heartbeat.try_lock();
                if let Err(e) = handle_heart_beat(o).await {
                    eprintln!("Error sending heartbeat: {:?}", e);
                }
            }
        });
        task.await
    }

    async fn handle_execution_requests<T>(
        self,
        context: ExecuteProvider<T>,
        receiver: &mut UnboundedReceiver<JupyterMessage>,
        execution_reply_sender: &UnboundedSender<JupyterMessage>,
    ) -> JupyterResult<()> {
        let mut execution_count = 1;
        loop {
            execution_count += 1;
            match receiver.recv().await {
                None => {}
                Some(s) => {
                    println!("Got execution request: {:?}", s);
                    if let Err(e) = execution_reply_sender.send(s) {
                        eprintln!("Error sending execution reply: {:?}", e);
                    }
                }
            }
        }
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

async fn handle_shell<'a, T>(
    mut executor: MutexGuard<'a, T>,
    mut io: MutexGuard<'a, Connection<PubSocket>>,
    mut connection: MutexGuard<'a, Connection<RouterSocket>>,
) -> JupyterResult<()>
where
    T: ExecuteContext,
{
    // Processing of every message should be enclosed between "busy" and "idle"
    // see https://jupyter-client.readthedocs.io/en/latest/messaging.html#messages-on-the-shell-router-dealer-channel
    // Jupiter Lab doesn't use the kernel until it received "idle" for kernel_info_request
    let zmq = JupyterMessage::read(&mut connection).await?;
    zmq.create_message(JupyterMessageType::Status).with_content(ExecutionState::new("busy")).send(&mut io).await?;
    match zmq.kind() {
        JupyterMessageType::KernelInfoRequest => {
            let cont = JupiterContent::build_kernel_info_reply(executor.deref());
            let reply = zmq.as_reply().with_content(cont);
            // println!("Sending kernel info reply: {:#?}", reply);
            reply.send(&mut connection).await?
        }
        JupyterMessageType::Custom(v) => {
            println!("Got custom message: {:?}", v);
        }
        _ => {
            println!("Got unknown message: {:?}", zmq);
        }
    }
    zmq.create_message(JupyterMessageType::Status).with_content(ExecutionState::new("idle")).send(&mut io).await?;
    println!("Got shell message: {:?}", zmq.kind());

    // connect.socket.send(ZmqMessage::from(b"ping".to_vec())).await?;
    Ok(())
}

async fn handle_heart_beat<'a>(lock: Result<MutexGuard<'a, Connection<RepSocket>>, TryLockError>) -> JupyterResult<()> {
    let mut connect = match lock {
        Ok(o) => o,
        Err(_) => return Ok(()),
    };
    let _ = connect.socket.recv().await?;
    connect.socket.send(ZmqMessage::from(b"ping".to_vec())).await?;
    Ok(())
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
    context: &Arc<std::sync::Mutex<ExecuteProvider<T>>>,
    message: JupyterMessage,
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