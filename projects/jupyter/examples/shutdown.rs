extern crate env_logger;
extern crate jupyter_client;
extern crate structopt;

use jupyter_client::commands::Command;
use jupyter_client::Client;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long = "restart", short = "r")]
    /// Restart the server
    restart: bool,
}

fn main() {
    env_logger::init();

    let args = Opt::from_args();

    let client = Client::existing().expect("creating jupyter connection");

    let command = Command::Shutdown {
        restart: args.restart,
    };
    let response = client
        .send_control_command(command)
        .expect("sending command");
    println!("Response: {:#?}", response);

    thread::sleep(Duration::from_secs(1));
}
