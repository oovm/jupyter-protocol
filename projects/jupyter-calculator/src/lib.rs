use crate::values::{test_mathml, test_url};
use clap_derive::{Parser, Subcommand};
use jupyter::{
    async_trait, ExecutionReply, ExecutionRequest, ExecutionResult, InstallAction, JupyterKernelProtocol, JupyterKernelSockets,
    JupyterResult, LanguageInfo, OpenAction, StartAction, UnboundedSender, UninstallAction, Value,
};
use jupyter_derive::{include_png32, include_png64};
use std::{path::PathBuf, str::FromStr};
mod values;
pub use crate::engine::{ElementaryAlgebra, Evaluator, Printer, SqrtAlgebra};
mod engine;

pub struct CalculatorContext {
    sockets: JupyterKernelSockets,
}

#[async_trait]
impl JupyterKernelProtocol for CalculatorContext {
    fn language_info(&self) -> LanguageInfo {
        let mut info = LanguageInfo::new("calculator", "Calculator")
            .with_file_extensions(".calc", "text/calculator")
            .with_version(env!("CARGO_PKG_VERSION"))
            .with_syntax("scala", "scala");
        info.png_32 = include_png32!();
        info.png_64 = include_png64!();
        info
    }

    async fn running(&mut self, code: ExecutionRequest) -> ExecutionReply {
        self.sockets.send_executed(true).await;
        self.sockets.send_executed(0).await;
        self.sockets.send_executed(-std::f64::consts::PI).await;
        self.sockets.send_executed('c').await;
        self.sockets.send_executed("string").await;
        let json = Value::from_str(include_str!("../package.json"));
        self.sockets.send_executed(json.expect("package.json is invalid")).await;
        self.sockets.send_executed(test_url()).await;
        self.sockets.send_executed(test_mathml()).await;
        // self.sockets.send_executed(test_svg()).await;

        ExecutionReply::new(true, code.execution_count)
    }
    async fn bind_execution_socket(&self, sender: UnboundedSender<ExecutionResult>) {
        self.sockets.bind_execution_socket(sender).await
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
        let config = CalculatorContext { sockets: JupyterKernelSockets::default() };
        match &self.command {
            JupyterCommands::Open(v) => v.run(),
            JupyterCommands::Start(v) => v.run(config),
            JupyterCommands::Install(v) => v.run(config),
            JupyterCommands::Uninstall(v) => v.run(config),
        }
    }
}
