use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Deserialize)]
pub struct ShutdownRequest {
    restart: bool,
}

#[derive(Clone, Debug)]
pub struct ShutdownReply {
    /// return true if restart, or false if finally shutdown
    pub restart: bool,
}

#[allow(dead_code)]
impl ShutdownRequest {
    /// will not send in face, will be killed with no response
    pub fn as_reply(&self) -> ShutdownReply {
        ShutdownReply { restart: true }
    }
}

impl Serialize for ShutdownRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ShutdownRequest", 2)?;
        s.serialize_field("status", "ok")?;
        s.serialize_field("restart", &self.restart)?;
        s.end()
    }
}
