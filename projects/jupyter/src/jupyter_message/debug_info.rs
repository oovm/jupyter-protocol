#![allow(non_snake_case)]

use crate::{
    client::ExecuteProvider,
    value_type::{InspectModule, InspectVariable, InspectVariableRequest},
    ExecutionResult, JupyterKernelProtocol, JupyterResult,
};
use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::{to_value, Value};
use serde_lsp::dap::{DebugCapability, Variable, VariablesArguments, VariablesResponseBody};
use std::{collections::HashMap, fmt::Formatter, ops::Deref};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DebugRequest {
    command: String,
    seq: u32,
    r#type: String,
    arguments: Value,
}

#[derive(Clone, Debug, Serialize)]
pub struct ModulesResponse {
    pub modules: Vec<InspectModule>,
    #[serde(rename = "totalModules")]
    pub total_modules: usize,
}

#[derive(Clone, Debug)]
pub struct DapResponse<T> {
    success: bool,
    command: String,
    request_seq: u32,
    body: T,
}

impl<T: Serialize> Serialize for DapResponse<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(5))?;
        s.serialize_entry("type", "response")?;
        s.serialize_entry("command", &self.command)?;
        s.serialize_entry("request_seq", &self.request_seq)?;
        s.serialize_entry("success", &self.success)?;
        s.serialize_entry("body", &self.body)?;
        s.end()
    }
}

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
impl<T> DapResponse<T> {
    pub fn success(request: &DebugRequest, body: T) -> JupyterResult<Value>
    where
        T: Serialize,
    {
        let item = Self { success: true, command: request.command.clone(), request_seq: request.seq, body };
        Ok(to_value(item)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebugSource {
    content: String,
}

impl DebugRequest {
    pub async fn as_reply<K: JupyterKernelProtocol>(&self, kernel: ExecuteProvider<K>) -> JupyterResult<Value> {
        match self.command.as_str() {
            "debugInfo" => {
                let mut start = kernel.debugging.lock().await;
                if *start {
                    DapResponse::success(self, DebugInfoResponse::new(true))
                }
                else {
                    *start = true;
                    DapResponse::success(self, DebugInfoResponse::new(false))
                }
            }
            "initialize" => DapResponse::success(self, DebugCapability::default()),
            // Root variable query event when first opened
            "inspectVariables" => {
                let runner = kernel.context.lock().await;
                DapResponse::success(self, make_variables_response(runner.inspect_variables(None)))
            }
            // Subquery event after manual click on variable
            "variables" => {
                let request = VariablesArguments::deserialize(&self.arguments)?;
                let runner = kernel.context.lock().await;
                let variables = runner.inspect_variables(Some(InspectVariableRequest {
                    id: request.variables_reference,
                    filter: request.filter,
                    start: request.start,
                    limit: request.count,
                }));
                DapResponse::success(self, make_variables_response(variables))
            }
            "richInspectVariables" => {
                let runner = kernel.context.lock().await;
                let result = runner.inspect_details(&InspectVariable::default());
                DapResponse::success(self, ExecutionResult::new(result.deref()))
            }
            "source" => {
                let runner = kernel.context.lock().await;
                let content = runner.inspect_sources();
                DapResponse::success(self, DebugSource { content })
            }
            "dumpCell" => DapResponse::success(self, DumpCell { sourcePath: "sourcePath".to_string() }),
            "modules" => {
                let runner = kernel.context.lock().await;
                let modules = runner.inspect_modules(0);
                let total_modules = modules.len();
                DapResponse::success(self, ModulesResponse { modules, total_modules })
            }
            "attach" => {
                tracing::error!("Unimplemented DAP command: attach");
                DapResponse::success(self, "")
            }
            _ => {
                tracing::error!("Unknown DAP command: {}\n{:#?}", self.command, self.arguments);
                Ok(Value::Null)
            }
        }
    }
}

fn make_variables_response(vars: Vec<InspectVariable>) -> VariablesResponseBody {
    let mut variables = Vec::with_capacity(vars.len());
    for var in vars {
        variables.push(Variable {
            name: var.name,
            value: var.value,
            typing: var.typing,
            evaluate_name: "".to_string(),
            variables_reference: var.id.map(|v| v.get()).unwrap_or(0),
            named_variables: var.named_variables,
            indexed_variables: var.indexed_variables,
            memory_reference: format!("{:x}", var.memory_reference),
        })
    }
    VariablesResponseBody { variables }
}

impl Default for DebugRequest {
    fn default() -> Self {
        Self { command: "".to_string(), seq: 0, r#type: "".to_string(), arguments: Value::Null }
    }
}

pub struct DebugInfoVisitor<'i> {
    wrapper: &'i mut DebugRequest,
}

impl<'de> Deserialize<'de> for DebugRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut out = Self::default();
        deserializer.deserialize_map(DebugInfoVisitor { wrapper: &mut out })?;
        Ok(out)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DebugInfoVisitor { wrapper: place })
    }
}

impl<'i, 'de> Visitor<'de> for DebugInfoVisitor<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("struct DebugInfo")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "command" => self.wrapper.command = map.next_value()?,
                "seq" => self.wrapper.seq = map.next_value()?,
                "type" => self.wrapper.r#type = map.next_value()?,
                "arguments" => self.wrapper.arguments = map.next_value()?,
                _ => {}
            }
        }
        Ok(())
    }
}
