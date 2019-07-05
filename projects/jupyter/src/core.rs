// Copyright 2020 The Evcxr Authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE
// or https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::{connection::Connection, errors::JupyterResult, jupyter_message::JupyterMessage, KernelControl};
use ariadne::sources;
use colored::*;
use crossbeam_channel::Select;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;

// Note, to avoid potential deadlocks, each thread should lock at most one mutex at a time.
#[derive(Clone)]
pub(crate) struct Server {
    iopub: Arc<Mutex<Connection<zeromq::PubSocket>>>,
    stdin: Arc<Mutex<Connection<zeromq::RouterSocket>>>,
    latest_execution_request: Arc<Mutex<Option<JupyterMessage>>>,
    shutdown_sender: Arc<Mutex<Option<crossbeam_channel::Sender<()>>>>,
    tokio_handle: tokio::runtime::Handle,
}

#[derive(Clone, Default)]
pub struct CommandContext {}

impl CommandContext {
    pub(crate) fn execute(&self, p0: &str) -> JupyterResult<()> {
        todo!()
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
        let mut heartbeat = bind_socket::<zeromq::RepSocket>(config, config.hb_port).await?;
        let shell_socket = bind_socket::<zeromq::RouterSocket>(config, config.shell_port).await?;
        let control_socket = bind_socket::<zeromq::RouterSocket>(config, config.control_port).await?;
        let stdin_socket = bind_socket::<zeromq::RouterSocket>(config, config.stdin_port).await?;
        let iopub_socket = bind_socket::<zeromq::PubSocket>(config, config.iopub_port).await?;
        let iopub = Arc::new(Mutex::new(iopub_socket));

        let (shutdown_sender, shutdown_receiver) = crossbeam_channel::unbounded();

        let server = Server {
            iopub,
            latest_execution_request: Arc::new(Mutex::new(None)),
            stdin: Arc::new(Mutex::new(stdin_socket)),
            shutdown_sender: Arc::new(Mutex::new(Some(shutdown_sender))),
            tokio_handle,
        };

        // let (execution_sender, mut execution_receiver) = tokio::sync::mpsc::unbounded_channel();
        // let (execution_response_sender, mut execution_response_receiver) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            if let Err(error) = Self::handle_hb(&mut heartbeat).await {
                eprintln!("hb error: {error:?}");
            }
        });
        let mut context = CommandContext::default();
        context.execute(":load_config")?;
        // let process_handle = context.process_handle();
        // let context = Arc::new(std::sync::Mutex::new(context));
        // {
        //     let server = server.clone();
        //     tokio::spawn(async move {
        //         if let Err(error) = server.handle_control(control_socket, process_handle).await {
        //             eprintln!("control error: {error:?}");
        //         }
        //     });
        // }
        {
            let context = context.clone();
            let server = server.clone();
            tokio::spawn(async move {
                // let result =
                //     server.handle_shell(shell_socket, &execution_sender, &mut execution_response_receiver, context).await;
                // if let Err(error) = result {
                //     eprintln!("shell error: {error:?}");
                // }
            });
        }
        {
            let server = server.clone();
            tokio::spawn(async move {
                // let result =
                //     server.handle_execution_requests(&context, &mut execution_receiver, &execution_response_sender).await;
                // if let Err(error) = result {
                //     eprintln!("execution error: {error:?}");
                // }
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

    async fn handle_hb(connection: &mut Connection<zeromq::RepSocket>) -> JupyterResult<()> {
        use zeromq::{SocketRecv, SocketSend};
        loop {
            connection.socket.recv().await?;
            connection.socket.send(zeromq::ZmqMessage::from(b"ping".to_vec())).await?;
        }
    }
}

impl ShutdownReceiver {
    async fn wait_for_shutdown(self) {
        let _ = tokio::task::spawn_blocking(move || self.recv.recv()).await;
    }
}

async fn comm_open(
    message: JupyterMessage,
    context: &Arc<std::sync::Mutex<CommandContext>>,
    iopub: Arc<Mutex<Connection<zeromq::PubSocket>>>,
) -> JupyterResult<()> {
    if message.target_name() == "evcxr-cargo-check" {
        let context = Arc::clone(context);
        tokio::spawn(async move {
            if let Some(code) = message.data()["code"].as_str() {
                // let data = cargo_check(code.to_owned(), context).await;
                // panic!("{}", data)

                // message
                //     .new_message("comm_msg")
                //     .without_parent_header()
                //     .with_content(response_content)
                //     .send(&mut *iopub.lock().await)
                //     .await
                //     .unwrap();
            }
            message.comm_close_message().send(&mut *iopub.lock().await).await.unwrap();
        });
        Ok(())
    }
    else {
        // Unrecognised comm target, just close the comm.
        message.comm_close_message().send(&mut *iopub.lock().await).await
    }
}

async fn bind_socket<S: zeromq::Socket>(config: &KernelControl, port: u16) -> JupyterResult<Connection<S>> {
    let endpoint = format!("{}://{}:{}", config.transport, config.ip, port);
    let mut socket = S::new();
    socket.bind(&endpoint).await?;
    Connection::new(socket, &config.key)
}

pub struct KernelInfo {
    protocol_version: String,
    implementation: String,
    implementation_version: String,
    language_info: LanguageInfo,
    banner: String,
    help_links: Vec<HelpLink>,
    status: String,
}

pub struct LanguageInfo {
    name: String,
    version: String,
    mimetype: String,
    file_extension: String,
    // Pygments lexer, for highlighting Only needed if it differs from the 'name' field.
    // see http://pygments.org/docs/lexers/#lexers-for-the-rust-language
    pygment_lexer: String,
    // Codemirror mode, for for highlighting in the notebook. Only needed if it differs from the 'name' field.
    // codemirror use text/x-rustsrc as mimetypes
    // see https://codemirror.net/mode/rust/
    codemirror_mode: String,
    nbconvert_exporter: String,
}

pub struct HelpLink {
    text: String,
    url: String,
}

impl KernelInfo {
    /// See [Kernel info documentation](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-info)
    pub fn rust() -> KernelInfo {
        KernelInfo {
            protocol_version: "5.3".to_owned(),
            implementation: env!("CARGO_PKG_NAME").to_owned(),
            implementation_version: env!("CARGO_PKG_VERSION").to_owned(),
            language_info: LanguageInfo {
                name: "Rust".to_owned(),
                version: "".to_owned(),
                mimetype: "text/rust".to_owned(),
                file_extension: ".rs".to_owned(),
                pygment_lexer: "rust".to_owned(),
                codemirror_mode: "rust".to_owned(),
                nbconvert_exporter: "rust".to_owned(),
            },
            banner: format!("EvCxR {} - Evaluation Context for Rust", env!("CARGO_PKG_VERSION")),
            help_links: vec![HelpLink {
                text: "Rust std docs".to_owned(),
                url: "https://doc.rust-lang.org/std/index.html".to_owned(),
            }],
            status: "ok".to_owned(),
        }
    }
}

async fn handle_completion_request(
    context: &Arc<std::sync::Mutex<CommandContext>>,
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
