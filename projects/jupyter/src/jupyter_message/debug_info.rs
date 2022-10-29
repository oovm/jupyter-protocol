use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::Value;
use std::fmt::Formatter;

// {
//     'type' : 'response',
//     'success' : bool,
//     'body' : {
//         'isStarted' : bool,  # whether the debugger is started,
//         'hashMethod' : str,  # the hash method for code cell. Default is 'Murmur2',
//         'hashSeed' : str,  # the seed for the hashing of code cells,
//         'tmpFilePrefix' : str,  # prefix for temporary file names
//         'tmpFileSuffix' : str,  # suffix for temporary file names
//         'breakpoints' : [  # breakpoints currently registered in the debugger.
//             {
//                 'source' : str,  # source file
//                 'breakpoints' : list(source_breakpoints)  # list of breakpoints for that source file
//             }
//         ],
//         'stoppedThreads' : list(int),  # threads in which the debugger is currently in a stopped state
//         'richRendering' : bool,  # whether the debugger supports rich rendering of variables
//         'exceptionPaths' : list(str),  # exception names used to match leaves or nodes in a tree of exception
//     }
// }
#[derive(Clone, Debug)]
pub struct DebugRequest {
    command: String,
    seq: u32,
    r#type: String,
    arguments: Value,
}

#[derive(Clone, Debug)]
pub enum DebugResponse {
    Custom(DapResponse<Value>),
    DebugInfo(DapResponse<DebugInfoResponseBody>),
}

impl Serialize for DebugResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DebugResponse::Custom(r) => r.serialize(serializer),
            DebugResponse::DebugInfo(r) => r.serialize(serializer),
        }
    }
}

impl Default for DebugResponse {
    fn default() -> Self {
        DebugResponse::Custom(DapResponse { r#type: "response".to_string(), success: true, body: Value::Null })
    }
}

#[derive(Clone, Debug)]
pub struct DapResponse<T> {
    r#type: String,
    success: bool,
    body: T,
}

impl<T: Serialize> Serialize for DapResponse<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry("type", &self.r#type)?;
        s.serialize_entry("success", &self.success)?;
        s.serialize_entry("body", &self.body)?;
        s.end()
    }
}

#[allow(non_snake_case)]
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
            hashSeed: "42".to_string(),
            tmpFilePrefix: "_tmp".to_string(),
            tmpFileSuffix: "".to_string(),
            breakpoints: vec![],
            stoppedThreads: vec![],
            richRendering: true,
            exceptionPaths: vec![],
        }
    }
}

impl DebugRequest {
    pub fn reply_debug_info(&self) -> DebugResponse {
        match self.command.as_str() {
            "debugInfo" => DebugResponse::DebugInfo(DapResponse {
                r#type: "response".to_string(),
                success: true,
                body: DebugInfoResponseBody::default(),
            }),
            _ => {
                tracing::error!("Unknown DAP command: {}", self.command);
                DebugResponse::default()
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
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key()? {
            match key {
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
