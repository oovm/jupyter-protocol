use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonInfoRequest {
    pub target_name: String,
}

#[derive(Clone, Debug)]
pub struct CommonInfoReply {
    success: bool,
}

impl CommonInfoRequest {
    pub fn as_reply(&self) -> CommonInfoReply {
        CommonInfoReply { success: true }
    }
    pub fn as_error(&self) -> CommonInfoReply {
        CommonInfoReply { success: false }
    }
}

impl Serialize for CommonInfoReply {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry("status", if self.success { "ok" } else { "error" })?;
        s.end()
    }
}
