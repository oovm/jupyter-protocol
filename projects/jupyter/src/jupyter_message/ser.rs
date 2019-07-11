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
