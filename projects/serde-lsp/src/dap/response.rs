use super::*;
use serde_json::{Error, Value};

#[derive(Clone, Debug)]
pub struct Response<T> {
    /// Sequence number of the corresponding request.
    pub request_sequence: usize,
    /// Outcome of the request.
    /// If true, the request was successful and the `body` attribute may contain
    /// the result of the request.
    /// If the value is false, the attribute `message` contains the error in short
    /// form and the `body` may contain additional information (see
    /// `ErrorResponse.body.error`).
    pub success: bool,
    /// The command requested.
    pub command: String,
    pub body: T,
}

impl<T> Serialize for Response<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(5))?;
        s.serialize_entry("type", "response")?;
        s.serialize_entry("command", &self.command)?;
        s.serialize_entry("request_seq", &self.request_sequence)?;
        s.serialize_entry("success", &self.success)?;
        s.serialize_entry("body", &self.body)?;
        s.end()
    }
}

impl<T> Response<T> {
    pub fn success(request: Request, body: T) -> Result<Value, Error>
    where
        T: Serialize,
    {
        let item = Self { request_sequence: request.sequence, success: true, command: request.command, body };
        serde_json::to_value(&item)
    }
}
