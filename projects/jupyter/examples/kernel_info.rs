extern crate env_logger;
extern crate jupyter_client;

use jupyter_client::commands::Command;
use jupyter_client::Client;

fn main() {
    env_logger::init();

    let client = Client::existing().expect("creating jupyter connection");

    let command = Command::KernelInfo;
    let response = client.send_shell_command(command).expect("sending command");
    println!("Response: {:#?}", response);
}
