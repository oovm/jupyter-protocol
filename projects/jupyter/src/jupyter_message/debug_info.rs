#![allow(non_snake_case)]
use crate::JupyterResult;
use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::{to_value, Value};
use std::fmt::Formatter;
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
    pub modules: Vec<Module>,
    pub totalModules: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct Module {
    pub id: u32,
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug)]
pub struct DapResponse<T> {
    success: bool,
    body: T,
}

impl<T: Serialize> Serialize for DapResponse<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry("type", "response")?;
        s.serialize_entry("success", &self.success)?;
        s.serialize_entry("body", &self.body)?;
        s.end()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DebugInfoResponseBody {
    /// whether the debugger is started
    isStarted: bool,
    /// the hash method for code cell. Default is 'Murmur2'
    hashMethod: String,
    /// the seed for the hashing of code cells
    hashSeed: String,
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

#[derive(Clone, Debug, Serialize)]
pub struct SourceBreakpoints {
    source: String,
    breakpoints: Vec<Breakpoint>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Breakpoint {}

impl Default for DebugInfoResponseBody {
    fn default() -> Self {
        Self {
            isStarted: true,
            hashMethod: "Murmur2".to_string(),
            hashSeed: Uuid::new_v4().to_string(),
            tmpFilePrefix: "_".to_string(),
            tmpFileSuffix: "".to_string(),
            breakpoints: vec![],
            stoppedThreads: vec![],
            richRendering: true,
            exceptionPaths: vec![],
        }
    }
}

impl Default for InspectVariable {
    fn default() -> Self {
        Self { name: "name".to_string(), variablesReference: 0, value: "value".to_string(), r#type: "Integer".to_string() }
    }
}

//         'variables' : [ # variables defined in the notebook.
//             {
//                 'name' : str,
//                 'variablesReference' : int,
//                 'value' : str,
//                 'type' : str
//             }
//         ]
#[derive(Clone, Debug, Serialize)]
pub struct InspectVariables {
    variables: Vec<InspectVariable>,
}

#[derive(Clone, Debug, Serialize)]
pub struct InspectVariable {
    name: String,
    variablesReference: i32,
    value: String,
    r#type: String,
}

impl InspectVariable {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into(), ..Self::default() }
    }
}

impl DebugRequest {
    pub fn as_reply(&self) -> JupyterResult<Value> {
        let value = match self.command.as_str() {
            "debugInfo" => to_value(DapResponse { success: true, body: DebugInfoResponseBody::default() })?,
            "inspectVariables" => to_value(DapResponse {
                success: true,
                body: InspectVariables { variables: vec![InspectVariable::default(), InspectVariable::new("112233")] },
            })?,
            "modules" => {
                let modules = vec![
                    Module { id: 1, name: "name".to_string(), path: "path".to_string() },
                    Module { id: 2, name: "111".to_string(), path: "222".to_string() },
                ];
                to_value(DapResponse { success: true, body: ModulesResponse { modules, totalModules: 2 } })?
            }
            _ => {
                tracing::error!("Unknown DAP command: {}", self.command);
                Value::Null
            }
        };
        Ok(value)
    }
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
