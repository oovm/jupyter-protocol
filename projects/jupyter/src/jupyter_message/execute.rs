use super::*;
use serde::{ser::SerializeMap, Serializer};

pub struct ExecutionGroup {
    pub message: JupyterMessage,
    pub request: ExecutionRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub code: String,
    pub silent: bool,
    pub store_history: bool,
    pub allow_stdin: bool,
    pub stop_on_error: bool,
    pub user_expressions: Value,
}
/// {
//   "source": "page",
//   # mime-bundle of data to display in the pager.
//   # Must include text/plain.
//   "data": mimebundle,
//   # line offset to start from
//   "start": int,
// }
#[derive(Clone, Debug, Serialize)]
pub struct ExecutionReply {
    execution_count: i32,
    data: Value,
    metadata: Value,
    // payload: Vec<ExecutionPayload>,
}

#[derive(Clone, Debug)]
pub enum ExecutionPayload {
    Page {
        mime: String,
        /// line offset to start from
        start: i32,
    },
    NextInput {
        text: String,
        replace: bool,
    },
}

impl Serialize for ExecutionPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ExecutionPayload::Page { mime: data, start } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("source", "page")?;
                map.serialize_entry("data", "text/plain")?;
                map.serialize_entry("start", start)?;
                map.end()
            }
            ExecutionPayload::NextInput { text, replace } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("source", "set_next_input")?;
                map.serialize_entry("text", text)?;
                map.serialize_entry("replace", replace)?;
                map.end()
            }
        }
    }
}

impl From<ExecutionReply> for JupiterContent {
    fn from(value: ExecutionReply) -> Self {
        JupiterContent::ExecutionReply(Box::new(value))
    }
}

impl ExecutionRequest {
    pub fn as_reply<T>(&self, count: i32, data: T) -> JupyterResult<ExecutionReply>
    where
        T: Serialize,
    {
        Ok(ExecutionReply {
            execution_count: count,
            data: serde_json::to_value(data)?,
            metadata: Value::Null,
            // payload: vec![
            //     ExecutionPayload::Page { mime: "".to_string(), start: 1 },
            //     ExecutionPayload::NextInput { text: "all".to_string(), replace: false },
            //     ExecutionPayload::NextInput { text: "other".to_string(), replace: false },
            // ],
        })
    }
    pub fn as_error(&self) {
        unimplemented!()
    }
}

impl ExecutionReply {
    pub fn with_meta<T>(self, data: T) -> JupyterResult<ExecutionReply>
    where
        T: Serialize,
    {
        Ok(ExecutionReply {
            execution_count: self.execution_count,
            data: self.data,
            metadata: serde_json::to_value(data)?,
            // payload: self.payload,
        })
    }
}
