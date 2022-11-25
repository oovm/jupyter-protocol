use crate::{
    connection::Connection,
    errors::JupyterResult,
    jupyter_message::{JupyterMessage, JupyterMessageType},
    CommonInfoRequest, ExecutionRequest, JupyterConnection, JupyterKernelProtocol, JupyterKernelSockets, KernelInfoReply,
};

use crate::commands::start::KernelControl;
use serde_json::Value;
use std::{sync::Arc, time::SystemTime};
use tokio::{sync::Mutex, task::JoinHandle};
use zeromq::{PubSocket, RepSocket, RouterSocket, Socket, SocketRecv, SocketSend, ZmqMessage};

// Note, to avoid potential deadlocks, each thread should lock at most one mutex at a time.
#[derive(Clone)]
#[allow(unused)]
pub(crate) struct SealedServer {
    heartbeat: Arc<Mutex<Connection<RepSocket>>>,
    iopub: Arc<Mutex<Connection<PubSocket>>>,
    stdin: Arc<Mutex<Connection<RouterSocket>>>,
    control: Arc<Mutex<Connection<RouterSocket>>>,
    shell_socket: Arc<Mutex<Connection<RouterSocket>>>,
    latest_execution_request: Arc<Mutex<Option<JupyterMessage>>>,
    // execution_request_receiver: Arc<Mutex<UnboundedReceiver<ExecutionResult>>>,
    shutdown_sender: Arc<Mutex<Option<crossbeam_channel::Sender<()>>>>,
    tokio_handle: tokio::runtime::Handle,
}

pub struct ExecuteProvider<T> {
    pub context: Arc<Mutex<T>>,
    pub debugging: Arc<Mutex<bool>>,
}

impl<T> Clone for ExecuteProvider<T> {
    fn clone(&self) -> Self {
        Self { context: self.context.clone(), debugging: Arc::new(Default::default()) }
    }
}

impl<T> ExecuteProvider<T> {
    pub fn new(context: T) -> Self
    where
        T: JupyterKernelProtocol + 'static,
    {
        Self { context: Arc::new(Mutex::new(context)), debugging: Arc::new(Default::default()) }
    }
}

struct ShutdownReceiver {
    // Note, this needs to be a crossbeam channel because
    // start_output_pass_through_thread selects on this and other crossbeam
    // channels.
    recv: crossbeam_channel::Receiver<()>,
}

impl SealedServer {
    pub(crate) fn run<T>(config: &KernelControl, server: T) -> JupyterResult<()>
    where
        T: JupyterKernelProtocol + 'static,
    {
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
            let shutdown_receiver = Self::start(config, handle, server).await?;
            shutdown_receiver.wait_for_shutdown().await;
            let result: JupyterResult<()> = Ok(());
            result
        })?;
        Ok(())
    }

    async fn start<T>(
        config: &KernelControl,
        tokio_handle: tokio::runtime::Handle,
        mut server: T,
    ) -> JupyterResult<ShutdownReceiver>
    where
        T: JupyterKernelProtocol + 'static,
    {
        let heartbeat = bind_socket::<RepSocket>(config, config.hb_port).await?;
        let shell_socket = bind_socket::<RouterSocket>(config, config.shell_port).await?;
        let control_socket = bind_socket::<RouterSocket>(config, config.control_port).await?;
        let stdin_socket = bind_socket::<RouterSocket>(config, config.stdin_port).await?;
        let io_pub_socket = bind_socket::<PubSocket>(config, config.iopub_port).await?;
        let io_pub = Arc::new(Mutex::new(io_pub_socket));
        let (shutdown_sender, shutdown_receiver) = crossbeam_channel::unbounded();
        let latest_execution_request = Arc::new(Mutex::new(None));

        let setup = JupyterConnection {
            boot_path: Default::default(),
            sockets: JupyterKernelSockets {
                // parent_buffer: latest_execution_request.clone(),
                // execute_channel: None,
                io_channel: Some(io_pub.clone()),
            },
        };
        server.connected(setup);
        // server.bind_execution_socket(execution_result_sender).await;
        let here = SealedServer {
            iopub: io_pub,
            heartbeat: Arc::new(Mutex::new(heartbeat)),
            latest_execution_request,
            // execution_request_receiver: Arc::new(Mutex::new(execution_receiver)),
            stdin: Arc::new(Mutex::new(stdin_socket)),
            control: Arc::new(Mutex::new(control_socket)),
            shutdown_sender: Arc::new(Mutex::new(Some(shutdown_sender))),
            tokio_handle,
            shell_socket: Arc::new(Mutex::new(shell_socket)),
        };
        let context = ExecuteProvider::new(server);
        here.clone().spawn_heart_beat();
        here.clone().spawn_shell_execution(context.clone());
        // server.clone().spawn_execution_queue(context.clone());
        here.clone().spawn_control(context.clone());
        here.clone().spawn_std_in(context.clone());
        Ok(ShutdownReceiver { recv: shutdown_receiver })
    }

    async fn signal_shutdown(&self) {
        // let msg = ShutdownReply::new();
        self.shutdown_sender.lock().await.take();
    }

    fn spawn_heart_beat(self) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                if let Err(e) = self.clone().handle_heart_beat().await {
                    tracing::warn!("Error sending heartbeat: {:?}", e);
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
        T: JupyterKernelProtocol + Send + 'static,
    {
        let mut count = 0;
        tokio::spawn(async move {
            tracing::info!("Shell Executor Spawned");
            loop {
                if let Err(e) = self.clone().handle_shell(executor.clone(), &mut count).await {
                    tracing::error!("Error sending shell execution: {:?}", e);
                }
            }
        })
    }
    async fn handle_shell<'a, T>(self, executor: ExecuteProvider<T>, count: &mut u32) -> JupyterResult<()>
    where
        T: JupyterKernelProtocol + Send + 'static,
    {
        // Processing of every message should be enclosed between "busy" and "idle"
        // see https://jupyter-client.readthedocs.io/en/latest/messaging.html#messages-on-the-shell-router-dealer-channel
        // Jupiter Lab doesn't use the kernel until it received "idle" for kernel_info_request
        let request = JupyterMessage::read(&mut &mut self.shell_socket.lock().await).await?;
        request.send_state(self.iopub.clone(), true).await?;
        match request.kind() {
            JupyterMessageType::KernelInfoRequest => {
                let info = executor.context.lock().await.language_info();
                let cont = KernelInfoReply::build(info);
                request.as_reply().with_content(cont)?.send_by(&mut &mut self.shell_socket.lock().await).await?
            }
            JupyterMessageType::ExecuteRequest => {
                let time = SystemTime::now();
                // *self.latest_execution_request.lock().await = Some(request);
                *count += 1;
                let mut task = request.recast::<ExecutionRequest>()?;
                task.execution_count = *count;
                task.header = request.clone();
                // reply busy event
                let mut runner = executor.context.lock().await;
                let reply = runner.running(task.clone()).await;
                // Check elapsed time
                match time.elapsed() {
                    Ok(o) => {
                        let escape = runner.running_time(o.as_secs_f64());
                        if !escape.is_empty() {
                            let time = task.as_result("text/html".to_string(), Value::String(escape)).with_count(*count);
                            request
                                .as_reply()
                                .with_message_type(JupyterMessageType::ExecuteResult)
                                .with_content(time)?
                                .send_by(&mut &mut self.iopub.lock().await)
                                .await?;
                        }
                    }
                    Err(_) => {}
                }
                // reply finish event
                request.as_reply().with_content(reply)?.send_by(&mut &mut self.shell_socket.lock().await).await?;
            }
            JupyterMessageType::CommonInfoRequest => {
                let task = request.recast::<CommonInfoRequest>()?;
                request.as_reply().with_content(task.as_reply())?.send_by(&mut &mut self.shell_socket.lock().await).await?;
            }
            JupyterMessageType::Custom(v) => {
                tracing::error!("Got unknown shell message: {:?}", v);
            }
            _ => {
                tracing::warn!("Got custom shell message: {:?}", request);
            }
        }
        request.send_state(self.iopub, false).await?;
        Ok(())
    }
    #[allow(dead_code)]
    fn spawn_execution_queue<T>(self, executor: ExecuteProvider<T>) -> JoinHandle<()>
    where
        T: JupyterKernelProtocol + Send + 'static,
    {
        let mut running_count = 0;
        tokio::spawn(async move {
            tracing::trace!("Queue Executor Spawned");
            loop {
                if let Err(e) = self.clone().handle_execution_queue(executor.clone(), running_count).await {
                    eprintln!("Error sending execution queue: {:?}", e);
                }
                running_count += 1;
            }
        })
    }
    #[allow(dead_code)]
    async fn handle_execution_queue<T>(self, _executor: ExecuteProvider<T>, _count: i32) -> JupyterResult<()>
    where
        T: JupyterKernelProtocol + Send + 'static,
    {
        // let io = self.iopub.try_lock()?;
        // let exec = self.execution_request_receiver.try_lock()?;
        todo!()
    }

    fn spawn_control<T>(self, executor: ExecuteProvider<T>) -> JoinHandle<()>
    where
        T: JupyterKernelProtocol + Send + 'static,
    {
        tokio::spawn(async move {
            tracing::info!("Control Executor Spawned");
            loop {
                if let Err(e) = self.clone().handle_control(executor.clone()).await {
                    tracing::error!("Error sending control execution: {:?}", e);
                }
            }
        })
    }
    async fn handle_control<'a, T>(self, executor: ExecuteProvider<T>) -> JupyterResult<()>
    where
        T: JupyterKernelProtocol + Send + 'static,
    {
        let control = &mut self.control.lock().await;
        let request = JupyterMessage::read(control).await?;
        // FUCK_ME: Time wasted here = two weeks
        // Who the hell designed this process and it's not mentioned in the docs!!!!!!!
        request.send_state(self.iopub.clone(), true).await?;
        // main reply
        match request.kind() {
            JupyterMessageType::KernelInfoRequest => {
                let info = executor.context.lock().await;
                let cont = KernelInfoReply::build(info.language_info());
                request.as_reply().with_content(cont)?.send_by(control).await?
            }

            JupyterMessageType::DebugRequest => {
                let result = request.debug_response(executor).await?;
                request.as_reply().with_content(result)?.send_by(control).await?;
            }
            JupyterMessageType::InterruptRequest => {
                let runner = executor.context.lock().await;
                match runner.interrupt_kernel() {
                    Some(_) => {
                        request
                            .as_reply()
                            .create_message(JupyterMessageType::StatusReply)
                            .with_content("ok")?
                            .send_by(control)
                            .await?
                    }
                    None => {
                        request
                            .as_reply()
                            .create_message(JupyterMessageType::StatusReply)
                            .with_content("ok")?
                            .send_by(control)
                            .await?
                    }
                }
            }
            JupyterMessageType::ShutdownRequest => self.signal_shutdown().await,
            JupyterMessageType::Custom(v) => {
                tracing::error!("Got unknown control message: {:#?}", v);
            }
            _ => {
                tracing::warn!("Got custom control message: {:#?}", request);
            }
        }
        request.send_state(self.iopub.clone(), false).await?;
        Ok(())
    }
    fn spawn_std_in<T>(self, executor: ExecuteProvider<T>) -> JoinHandle<()>
    where
        T: JupyterKernelProtocol + Send + 'static,
    {
        tokio::spawn(async move {
            tracing::info!("IO Executor Spawned");
            loop {
                if let Err(e) = self.clone().handle_std_in(executor.clone()).await {
                    tracing::error!("Error sending io execution: {:?}", e);
                }
            }
        })
    }
    async fn handle_std_in<'a, T>(self, _: ExecuteProvider<T>) -> JupyterResult<()>
    where
        T: JupyterKernelProtocol + Send + 'static,
    {
        let io = &mut self.stdin.lock().await;
        let request = JupyterMessage::read(io).await?;
        match request.kind() {
            JupyterMessageType::Custom(v) => {
                tracing::error!("Got unknown io message: {:?}", v);
            }
            _ => {
                tracing::warn!("Got custom io message: {:?}", request);
            }
        }
        Ok(())
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
