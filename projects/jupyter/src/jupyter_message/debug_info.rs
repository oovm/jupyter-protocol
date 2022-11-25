#![allow(non_snake_case)]

use crate::{
    client::ExecuteProvider,
    jupyter_message::JupyterMessage,
    value_type::{InspectModule, InspectVariable, InspectVariableRequest},
    ExecutionResult, JupyterKernelProtocol, JupyterResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_lsp::dap::{
    DebugCapability, Module, ModulesResponseBody, Request, Response, Variable, VariablesArguments, VariablesResponseBody,
};
use std::{collections::HashMap, ops::Deref};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct DebugInfoResponse {
    /// whether the debugger is started
    isStarted: bool,
    /// the hash method for code cell. Default is 'Murmur2'
    hashMethod: String,
    /// the seed for the hashing of code cells
    hashSeed: u32,
    /// prefix for temporary file names
    tmpFilePrefix: String,
    /// suffix for temporary file names
    tmpFileSuffix: String,
    /// breakpoints currently registered in the debugger
    breakpoints: Vec<SourceBreakpoints>,
    /// threads in which the debugger is currently in a stopped state
    stoppedThreads: Vec<i32>,
    /// whether the debugger supports rich rendering of variables
    richRendering: bool,
    /// exception names used to match leaves or nodes in a tree of exception
    exceptionPaths: Vec<String>,
}

impl DebugInfoResponse {
    pub fn new(start: bool) -> DebugInfoResponse {
        Self { isStarted: start, ..Default::default() }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SourceBreakpoints {
    source: String,
    breakpoints: Vec<Breakpoint>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Breakpoint {}

impl Default for DebugInfoResponse {
    fn default() -> Self {
        Self {
            isStarted: false,
            hashMethod: "Murmur2".to_string(),
            hashSeed: Uuid::new_v4().as_u128() as u32,
            tmpFilePrefix: "".to_string(),
            tmpFileSuffix: "".to_string(),
            breakpoints: vec![],
            stoppedThreads: vec![],
            richRendering: true,
            exceptionPaths: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DumpCell {
    sourcePath: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct RichInspectVariables {
    data: HashMap<String, String>,
    metadata: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebugSource {
    content: String,
}

impl From<InspectModule> for Module {
    fn from(value: InspectModule) -> Self {
        Module {
            id: value.id,
            name: value.name,
            path: value.path,
            is_optimized: false,
            is_user_code: false,
            version: "".to_string(),
        }
    }
}

impl From<InspectVariable> for Variable {
    fn from(value: InspectVariable) -> Self {
        Variable {
            name: value.name,
            value: value.value,
            typing: value.typing,
            evaluate_name: "".to_string(),
            variables_reference: value.id.map(|v| v.get()).unwrap_or(0),
            named_variables: value.named_variables,
            indexed_variables: value.indexed_variables,
            memory_reference: format!("{:x}", value.memory_reference),
        }
    }
}

impl JupyterMessage {
    pub(crate) async fn debug_response<K: JupyterKernelProtocol>(&self, kernel: ExecuteProvider<K>) -> JupyterResult<Value> {
        let request = self.recast::<Request>()?;
        let response = match request.command.as_str() {
            "debugInfo" => {
                let mut start = kernel.debugging.lock().await;
                if *start {
                    Response::success(request, DebugInfoResponse::new(true))?
                }
                else {
                    *start = true;
                    Response::success(request, DebugInfoResponse::new(false))?
                }
            }
            "initialize" => Response::success(request, DebugCapability::default())?,
            // Root variable query event when first opened
            "inspectVariables" => {
                let runner = kernel.context.lock().await;
                Response::success(request, VariablesResponseBody::from_iter(runner.inspect_variables(None)))?
            }
            // Subquery event after manual click on variable
            "variables" => {
                let args = request.recast::<VariablesArguments>()?;
                let runner = kernel.context.lock().await;
                let variables = runner.inspect_variables(Some(InspectVariableRequest {
                    id: args.variables_reference,
                    filter: args.filter,
                    start: args.start,
                    limit: args.count,
                }));
                Response::success(request, VariablesResponseBody::from_iter(variables))?
            }
            "richInspectVariables" => {
                let runner = kernel.context.lock().await;
                let result = runner.inspect_details(&InspectVariable::default());
                Response::success(request, ExecutionResult::new(result.deref()))?
            }
            "source" => {
                let runner = kernel.context.lock().await;
                let content = runner.inspect_sources();
                Response::success(request, DebugSource { content })?
            }
            "dumpCell" => Response::success(request, DumpCell { sourcePath: "sourcePath".to_string() })?,
            "modules" => {
                let runner = kernel.context.lock().await;
                let modules = runner.inspect_modules(0);
                Response::success(request, ModulesResponseBody::from_iter(modules))?
            }
            "attach" => {
                tracing::error!("Unimplemented DAP command: attach");
                Response::success(request, "")?
            }
            _ => {
                tracing::error!("Unknown DAP command: {}\n{:#?}", request.command, request.arguments);
                Value::Null
            }
        };
        Ok(response)
    }
}
