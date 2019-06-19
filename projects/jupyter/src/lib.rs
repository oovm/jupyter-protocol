#![deny(missing_docs)]
// TODO
/*! Jupyter Client

A Rust implementation of the [Jupyter client][jupyter-client].

## Examples

```no_run
extern crate jupyter_client;

# use std::collections::HashMap;
# use std::thread;
use jupyter_client::Client;
use jupyter_client::commands::Command;
# use jupyter_client::Result;
# fn main() -> Result<()> {

let client = Client::existing()?;

// Set up the heartbeat watcher
let hb_receiver = client.heartbeat()?;
thread::spawn(move || {
    for _ in hb_receiver {
        println!("Received heartbeat from kernel");
    }
});

// Spawn an IOPub watcher
let receiver = client.iopub_subscribe()?;
thread::spawn(move || {
    for msg in receiver {
        println!("Received message from kernel: {:#?}", msg);
    }
});

// Command to run
let command = Command::Execute {
    code: "a = 10".to_string(),
    silent: false,
    store_history: true,
    user_expressions: HashMap::new(),
    allow_stdin: true,
    stop_on_error: false,
};

// Run some code on the kernel
let response = client.send_shell_command(command)?;
# Ok(())
# }
```

[jupyter-client]: https://github.com/jupyter/jupyter_client
*/

extern crate chrono;
extern crate dirs;
extern crate failure;
extern crate glob;
extern crate hex;
extern crate hmac;
extern crate log;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;
extern crate uuid;
extern crate zmq;

#[cfg(test)]
extern crate crypto_mac;
#[cfg(test)]
extern crate digest;
#[cfg(test)]
extern crate generic_array;

#[cfg(test)]
#[macro_use]
mod test_helpers;

mod client;
pub mod commands;
mod connection_config;
mod errors;
mod header;
mod metadata;
mod paths;
pub mod responses;
mod signatures;
mod socket;
mod wire;

pub use crate::client::Client;
pub use crate::errors::Result;
