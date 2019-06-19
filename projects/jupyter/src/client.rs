use crate::commands::Command;
use crate::connection_config::ConnectionConfig;
use crate::errors::Result;
use crate::paths::jupyter_runtime_dir;
use crate::responses::Response;
use crate::signatures::HmacSha256;
use failure::format_err;
use glob::glob;
use hmac::Mac;
use log::{debug, trace};
use std::env::current_dir;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::socket::Socket;

fn find_connection_file<S>(glob_pattern: S, paths: Option<Vec<PathBuf>>) -> Option<PathBuf>
where
    S: Into<String>,
{
    let paths = paths.unwrap_or_else(|| {
        vec![
            current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            jupyter_runtime_dir(),
        ]
    });
    trace!("connection file paths to search: {:?}", paths);

    let glob_pattern = glob_pattern.into();

    for path in paths.into_iter() {
        let pattern = path.join(&glob_pattern);
        trace!("glob pattern: {:?}", pattern);
        let matches = glob(pattern.to_str().unwrap()).unwrap();
        let mut matches: Vec<PathBuf> = matches.map(|m| m.unwrap()).collect();
        trace!("matches: {:?}", matches);
        if !matches.is_empty() {
            matches.sort_by_key(|p| {
                let metadata = fs::metadata(p).unwrap();
                metadata.modified().unwrap()
            });
            trace!("sorted matches: {:#?}", matches);
            return Some(matches.last().unwrap().clone());
        }
    }
    None
}

/** The main `Client` struct.

This handles communication between the user's code, and the kernel itself. It abstracts the
conversion of messages to and from the [on-the-wire format][wire-format].

## Construction methods

- [`existing`][existing]: looks for the latest connection file and tries to connect
- [`from_reader`][from_reader]: reads connection details from a reader

## Communication with kernels

- [`send_shell_command`][send-shell-command]: send a shell command (like running a cell's contents)
- [`send_control_command`][send-control-command]: send an important shell command
- [`iopub_subscribe`][iopub-subscribe]: subscribe to published information from the kernel
- [`heartbeat_every`][heartbeat-every]: control the heartbeat and find out if the kernel dies
- [`heartbeat`][heartbeat]: send a heartbeat every second


[wire-format]: https://jupyter-client.readthedocs.io/en/stable/messaging.html#the-wire-protocol
[existing]: #method.existing
[from_reader]: #method.from_reader
[send-shell-command]: #method.send_shell_command
[send-control-command]: #method.send_control_command
[iopub-subscribe]: #method.iopub_subscribe
[heartbeat-every]: #method.heartbeat_every
[heartbeat]: #method.heartbeat
*/
pub struct Client {
    shell_socket: Socket,
    control_socket: Socket,
    iopub_socket: Arc<Mutex<Socket>>,
    heartbeat_socket: Arc<Mutex<Socket>>,
    auth: HmacSha256,
}

impl Client {
    /** Connect to the latest existing connection info file.

    This searches the standard runtime path for the latest kernel config file by last-modified
    time. This is then loaded by using [`from_reader`](#method.from_reader).

    ```no_run
    # use jupyter_client::{Result, Client};
    # fn main() -> Result<()> {
    let client = Client::existing()?;
    # Ok(())
    # }
    ```
     */
    pub fn existing() -> Result<Self> {
        use std::fs::File;

        find_connection_file("kernel-*.json", None)
            .ok_or_else(|| format_err!("no connection file found"))
            .and_then(|filename| {
                debug!("found connection file {:?}", filename);
                let f = File::open(filename)?;
                Self::from_reader(f)
            })
    }

    /** Connect to a kernel with a definition from a specific connection info file.

    This takes an [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) implementor
    (e.g. a [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html)).

    ```no_run
    # use jupyter_client::{Result, Client};
    # use std::fs::File;
    # use std::io::Read;
    # fn main() -> Result<()> {
    # let file = File::open("")?;
    // let file = File::open(...)?;
    let client = Client::from_reader(file)?;
    # Ok(())
    # }
    ```
    */
    pub fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: Read,
    {
        let config: ConnectionConfig = ConnectionConfig::from_reader(reader)?;
        let auth = HmacSha256::new_varkey(config.key.as_bytes())
            .map_err(|e| format_err!("Error constructing HMAC: {:?}", e))?;

        let ctx = zmq::Context::new();

        let shell_socket = Socket::new_shell(&ctx, &config)?;
        let control_socket = Socket::new_control(&ctx, &config)?;
        let iopub_socket = Socket::new_iopub(&ctx, &config)?;
        let heartbeat_socket = Socket::new_heartbeat(&ctx, &config)?;

        Ok(Client {
            shell_socket,
            control_socket,
            iopub_socket: Arc::new(Mutex::new(iopub_socket)),
            heartbeat_socket: Arc::new(Mutex::new(heartbeat_socket)),
            auth: auth,
        })
    }

    /** Send a shell command to the kernel.
     */
    pub fn send_shell_command(&self, command: Command) -> Result<Response> {
        debug!("Sending shell command: {:?}", command);
        self.send_command_to_socket(command, &self.shell_socket)
    }

    /** Send a control command to the kernel.
     */
    pub fn send_control_command(&self, command: Command) -> Result<Response> {
        debug!("Sending control command: {:?}", command);
        self.send_command_to_socket(command, &self.control_socket)
    }

    fn send_command_to_socket(&self, command: Command, socket: &Socket) -> Result<Response> {
        let wire = command.into_wire(self.auth.clone())?;
        socket.send_wire(wire)?;
        let resp_wire = socket.recv_wire(self.auth.clone())?;
        resp_wire.into_response()
    }

    /** Subscribe to IOPub messages.
     */
    pub fn iopub_subscribe(&self) -> Result<Receiver<Response>> {
        let (tx, rx) = mpsc::channel();
        let socket = self.iopub_socket.clone();
        let auth = self.auth.clone();

        thread::spawn(move || loop {
            let socket = socket.lock().unwrap();
            let wire = socket.recv_wire(auth.clone()).unwrap();
            let msg = wire.into_response().unwrap();
            tx.send(msg).unwrap();
        });

        Ok(rx)
    }

    /** Subscribe to heartbeat messages on a given duration.
     */
    pub fn heartbeat_every(&self, seconds: Duration) -> Result<Receiver<()>> {
        let (tx, rx) = mpsc::channel();
        let socket = self.heartbeat_socket.clone();

        thread::spawn(move || loop {
            let socket = socket.lock().unwrap();
            let _msg = socket.heartbeat().unwrap();
            tx.send(()).unwrap();
            thread::sleep(seconds);
        });
        Ok(rx)
    }

    /** Subscribe to heartbeat messages every second.
     */
    pub fn heartbeat(&self) -> Result<Receiver<()>> {
        self.heartbeat_every(Duration::from_secs(1))
    }
}
