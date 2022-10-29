use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonInfoRequest {
    pub target_name: String,
}

#[derive(Clone, Debug)]
pub struct CommonInfoReply {}

impl From<CommonInfoReply> for JupiterContent {
    fn from(value: CommonInfoReply) -> Self {
        JupiterContent::CommonInfoReply(Box::new(value))
    }
}

impl CommonInfoRequest {
    pub fn as_reply(&self) -> CommonInfoReply {
        CommonInfoReply {}
    }
    pub fn as_error(&self) {
        unimplemented!()
    }
}

impl Serialize for CommonInfoReply {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry("status", "ok")?;
        s.end()
    }
}
