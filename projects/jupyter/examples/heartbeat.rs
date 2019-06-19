extern crate env_logger;
extern crate jupyter_client;

use jupyter_client::Client;

fn main() {
    env_logger::init();

    let client = Client::existing().expect("creating jupyter connection");

    let receiver = client.heartbeat().unwrap();
    for msg in receiver.iter().take(5) {
        println!("{:?}", msg);
    }
}
