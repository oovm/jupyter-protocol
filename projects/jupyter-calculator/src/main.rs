use calculator::JupyterApplication;
use clap::Parser;
use jupyter::JupyterResult;

fn main() -> JupyterResult<()> {
    tracing_subscriber::fmt::init();
    JupyterApplication::parse().run()
}
