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
pub struct DumpCell {
    sourcePath: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct InspectVariable {
    name: String,
    variablesReference: i32,
    value: String,
    r#type: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct RichInspectVariables {
    variableName: String,
}

#[derive(Clone, Debug, Serialize)]
struct Variable {
    /// The variable's name.
    name: String,
    /**
     * The variable's value.
     * This can be a multi-line text, e.g. for a function the body of a function.
     * For structured variables (which do not have a simple value), it is
     * recommended to provide a one-line representation of the structured object.
     * This helps to identify the structured object in the collapsed state when
     * its children are not yet visible.
     * An empty string can be used if no value should be shown in the UI.
     */
    value: String,
    /**
     * The type of the variable's value. Typically shown in the UI when hovering
     * over the value.
     * This attribute should only be returned by a debug adapter if the
     * corresponding capability `supportsVariableType` is true.
     */
    r#type: String,
    //   /**
//    * Properties of a variable that can be used to determine how to render the
//    * variable in the UI.
//    */
//   presentationHint?: VariablePresentationHint;
//
    /**
     * The evaluatable name of this variable which can be passed to the `evaluate`
     * request to fetch the variable's value.
     */
    evaluateName: String,

    /**
     * If `variablesReference` is > 0, the variable is structured and its children
     * can be retrieved by passing `variablesReference` to the `variables` request
     * as long as execution remains suspended. See 'Lifetime of Object References'
     * in the Overview section for details.
     */
    variablesReference: u32,

    /**
     * The number of named child variables.
     * The client can use this information to present the children in a paged UI
     * and fetch them in chunks.
     */
    namedVariables: u32,

    /**
     * The number of indexed child variables.
     * The client can use this information to present the children in a paged UI
     * and fetch them in chunks.
     */
    indexedVariables: u32,
    /**
     * The memory reference for the variable if the variable represents executable
     * code, such as a function pointer.
     * This attribute is only required if the corresponding capability
     * `supportsMemoryReferences` is true.
     */
    memoryReference: String,
}

impl InspectVariable {
    pub fn new<T>(name: T) -> Self
        where
            T: Into<String>,
    {
        Self { name: name.into(), ..Self::default() }
    }
}

impl<T> DapResponse<T> {
    pub fn success(request: &DebugRequest, body: T) -> JupyterResult<Value>
        where T: Serialize
    {
        let item = Self { success: true, command: request.command.clone(), request_seq: request.seq, body };
        Ok(to_value(item)?)
    }
}


impl DebugRequest {
    pub fn as_reply(&self) -> JupyterResult<Value> {
        match self.command.as_str() {
            "debugInfo" => DapResponse::success(self, DebugInfoResponseBody::default()),
            "inspectVariables" => DapResponse::success(self, vec![InspectVariable::default(), InspectVariable::new("112233")]),
            "source" => {
                Ok(Value::Null)
            }
            "richInspectVariables" => {
                DapResponse::success(self, RichInspectVariables { variableName: "variableName".to_string() })
            }
            "variables" => {
                DapResponse::success(self, vec![Variable {
                    name: "name".to_string(),
                    value: "value".to_string(),
                    r#type: "type".to_string(),
                    evaluateName: "evaluateName".to_string(),
                    variablesReference: 11,
                    namedVariables: 22,
                    indexedVariables: 33,
                    memoryReference: "memoryReference".to_string(),
                }])
            }
            "dumpCell" => {
                DapResponse::success(self, DumpCell { sourcePath: "sourcePath".to_string() })
            }
            "modules" => {
                let modules = vec![
                    Module { id: 1, name: "name".to_string(), path: "path".to_string() },
                    Module { id: 2, name: "111".to_string(), path: "222".to_string() },
                ];
                DapResponse::success(self, ModulesResponse { modules, totalModules: 2 })
            }

            _ => {
                tracing::error!("Unknown DAP command: {}", self.command);
                Ok(Value::Null
                )
            }
        }
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
