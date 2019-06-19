use crate::errors::Result;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConnectionConfig {
    pub shell_port: u32,
    pub iopub_port: u32,
    pub stdin_port: u32,
    pub control_port: u32,
    pub hb_port: u32,
    pub ip: String,
    pub key: String,
    pub transport: String,
    pub signature_scheme: String,
    pub kernel_name: String,
}

impl ConnectionConfig {
    pub(crate) fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(reader).map_err(From::from)
    }
}
