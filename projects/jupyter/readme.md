# Easy Jupyter Client for your Language


```rust
use clap::Parser;
use clap_derive::{Parser, Subcommand};
use jupyter::{InstallAction, JupyterResult, OpenAction, StartAction, UninstallAction};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct JupyterApplication {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: JupyterCommands,
}

#[derive(Subcommand)]
enum JupyterCommands {
    Open(Box<OpenAction>),
    Start(Box<StartAction>),
    Install(Box<InstallAction>),
    Uninstall(Box<UninstallAction>),
}

impl JupyterApplication {
    pub fn run(&self) -> JupyterResult<()> {
        match &self.command {
            JupyterCommands::Open(v) => v.run(),
            JupyterCommands::Start(v) => v.run(),
            JupyterCommands::Install(v) => v.run(),
            JupyterCommands::Uninstall(v) => v.run(),
        }
    }
}

fn main() -> JupyterResult<()> {
    let app = JupyterApplication::parse();
    app.run()
}
```