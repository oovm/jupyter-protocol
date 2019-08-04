use clap::Parser;
use clap_derive::{Parser, Subcommand};
use jupyter::{
    async_trait, ExecutionReply, ExecutionRequest, InstallAction, JupyterResult, JupyterServerProtocol, LanguageInfo,
    OpenAction, StartAction, UninstallAction,
};
use std::path::PathBuf;

pub struct CalculatorContext;

#[async_trait]
impl JupyterServerProtocol for CalculatorContext {
    fn language_info(&self) -> LanguageInfo {
        LanguageInfo {
            language: "Calculate".to_string(),
            png_64: &[],
            png_32: &[],
            language_key: "calc".to_string(),
            file_extensions: ".calc".to_string(),
        }
    }

    async fn running(&mut self, code: ExecutionRequest) -> ExecutionReply {
        todo!()
    }
}

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
        let config = CalculatorContext {};
        match &self.command {
            JupyterCommands::Open(v) => v.run(),
            JupyterCommands::Start(v) => v.run(config),
            JupyterCommands::Install(v) => v.run(config),
            JupyterCommands::Uninstall(v) => v.run(config),
        }
    }
}

fn main() -> JupyterResult<()> {
    let app = JupyterApplication::parse();
    app.run()
}
