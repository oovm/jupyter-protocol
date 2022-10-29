use super::*;
use serde::{ser::SerializeStruct, Serializer};

impl Serialize for JupyterMessageHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JupyterMessageHeader", 6)?;
        if self.msg_type.is_empty() {
            state.end()
        }
        else {
            state.serialize_field("date", &self.date.to_rfc3339())?;
            state.serialize_field("msg_id", &self.msg_id)?;
            state.serialize_field("msg_type", &self.msg_type)?;
            state.serialize_field("session", &self.session)?;
            state.serialize_field("username", &self.username)?;
            state.serialize_field("version", &self.version)?;
            state.end()
        }
    }
}

impl Serialize for JupyterMessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Serialize for JupiterContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            JupiterContent::KernelInfoReply(v) => v.serialize(serializer),
            JupiterContent::Custom(v) => v.serialize(serializer),
            JupiterContent::State(v) => v.serialize(serializer),
            JupiterContent::ExecutionResult(v) => v.serialize(serializer),
            JupiterContent::ExecutionReply(v) => v.serialize(serializer),
            JupiterContent::DebugReply(v) => v.serialize(serializer),
            JupiterContent::CommonInfoReply(v) => v.serialize(serializer),
            JupiterContent::ExecutionRequest(_) => unreachable!(),
            JupiterContent::CommonInfoRequest(_) => unreachable!(),
            JupiterContent::ShutdownRequest(_) => unreachable!(),
            JupiterContent::DebugInfoRequest(_) => unreachable!(),
        }
    }
}
