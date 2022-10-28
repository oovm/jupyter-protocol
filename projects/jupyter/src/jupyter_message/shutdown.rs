use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Deserialize)]
pub struct ShutdownRequest {
    restart: bool,
}

#[derive(Clone, Debug)]
pub struct ShutdownReply {
    restart: bool,
}

impl ShutdownRequest {
    pub fn as_reply(&self) -> ShutdownReply {
        ShutdownReply { restart: self.restart }
    }
}

impl Serialize for ShutdownRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ShutdownRequest", 1)?;
        s.serialize_field("restart", &self.restart)?;
        s.end()
    }
}
