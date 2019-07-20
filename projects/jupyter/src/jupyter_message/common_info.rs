use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonInfoRequest {
    pub target_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonInfoReply {
    status: String,
}

impl From<CommonInfoReply> for JupiterContent {
    fn from(value: CommonInfoReply) -> Self {
        todo!()
    }
}

impl CommonInfoRequest {
    pub fn as_reply<T>(&self, data: T) -> JupyterResult<CommonInfoReply>
    where
        T: Serialize,
    {
        todo!()
    }
    pub fn as_error(&self) {
        unimplemented!()
    }
}

impl CommonInfoReply {
    pub fn with_meta<T>(self, data: T) -> JupyterResult<CommonInfoReply>
    where
        T: Serialize,
    {
        todo!()
    }
}
