use clap::Parser;
use clap_derive::{Parser, Subcommand};
use jupyter::{async_trait, ExecuteContext, Executed, ExecutionRequest, InstallAction, JupyterResult, LanguageInfo, OpenAction, Serialize, StartAction, to_value, UninstallAction, Value};
use std::path::PathBuf;
use rhai::{Engine, EvalAltResult};


pub struct JupyterRhai {
    engine: Engine,
}

pub struct RhaiExecuted<T> {
    result: Result<T, String>,
}

#[async_trait]
impl ExecuteContext for JupyterRhai {
    type Executed = RhaiExecuted<Value>;

    fn language_info(&self) -> LanguageInfo {
        LanguageInfo {
            language: "Rhai".to_string(),
            language_key: "rhai".to_string(),
            file_extensions: ".rhai".to_string(),
        }
    }

    async fn running(&mut self, code: ExecutionRequest) -> Vec<Self::Executed> {
        match self.engine.eval_expression(&code.code) {
            Ok(v) => vec![RhaiExecuted {
                result: Ok(v),
            }],
            Err(e) => vec![RhaiExecuted {
                result: Err(format!("{}", e)),
            }],
        }
    }
    fn running_time(&self, _: f64) -> String {
        String::new()
    }
}

impl<T> Executed for RhaiExecuted<T> where T: Serialize + Send {
    fn mime_type(&self) -> String {
        "text/plain".to_string()
    }

    fn as_json(&self) -> Value {
        match &self.result {
            Ok(v) => match to_value(v) {
                Ok(o) => {
                    o
                }
                Err(e) => {
                    Value::String(format!("{}", e))
                }
            },
            Err(e) => Value::String(format!("{}", e)),
        }
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
        let mut config = JupyterRhai { engine: Engine::RAW }.language_info();
        match &self.command {
            JupyterCommands::Open(v) => v.run(),
            JupyterCommands::Start(v) => v.run(),
            JupyterCommands::Install(v) => v.run(&config),
            JupyterCommands::Uninstall(v) => v.run(),
        }
    }
}

fn main() -> JupyterResult<()> {
    let app = JupyterApplication::parse();
    app.run()
}
