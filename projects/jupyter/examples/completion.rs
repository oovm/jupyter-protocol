extern crate env_logger;
extern crate jupyter_client;

use jupyter_client::commands::Command;
use jupyter_client::Client;
use std::collections::HashMap;

fn main() {
    env_logger::init();

    let client = Client::existing().expect("creating jupyter connection");

    // Set up some previous code
    let code = r#"class Foo(object):
    def bar(self):
        return 10

    def baz(self):
        return 20
"#
    .to_string();
    let prep_cmd = Command::Execute {
        code: code,
        silent: false,
        store_history: true,
        user_expressions: HashMap::new(),
        allow_stdin: true,
        stop_on_error: false,
    };

    client
        .send_shell_command(prep_cmd)
        .expect("sending command");

    let prep_cmd = Command::Execute {
        code: "a = Foo()".to_string(),
        silent: false,
        store_history: true,
        user_expressions: HashMap::new(),
        allow_stdin: true,
        stop_on_error: false,
    };

    client
        .send_shell_command(prep_cmd)
        .expect("sending command");

    let command = Command::Complete {
        code: "a.".to_string(),
        cursor_pos: 2,
    };
    let response = client.send_shell_command(command).expect("sending command");
    println!("Response: {:#?}", response);
}
