use crate::errors::Result;
use crate::header::Header;
use crate::metadata::Metadata;
use crate::responses::*;
use crate::signatures::sign;
use failure::{bail, format_err};
use hmac::Mac;
use log::{debug, trace};
use serde_json::Value;
use std::fmt::Debug;

type Part = Vec<u8>;

static DELIMITER: &[u8] = b"<IDS|MSG>";

#[derive(Debug)]
pub(crate) struct WireMessage<M: Mac + Debug> {
    pub(crate) header: Part,
    pub(crate) parent_header: Part,
    pub(crate) metadata: Part,
    pub(crate) content: Part,
    pub(crate) auth: M,
}

impl<M: Mac + Debug> WireMessage<M> {
    pub(crate) fn from_raw_response(raw: Vec<Vec<u8>>, auth: M) -> Result<Self> {
        trace!("raw response: {:?}", raw);
        let delim_idx = raw
            .iter()
            .position(|r| String::from_utf8(r.to_vec()).unwrap() == "<IDS|MSG>")
            .ok_or_else(|| format_err!("cannot find delimiter in response"))?;

        debug!(
            "identities: {:#?}",
            raw[..delim_idx]
                .iter()
                .map(|b| std::str::from_utf8(b))
                .collect::<Vec<_>>()
        );

        // Check the signature
        let signature = String::from_utf8_lossy(&raw[delim_idx + 1]);
        let msg_frames = &raw[delim_idx + 2..];
        let check_sig = sign(msg_frames, auth.clone());

        if check_sig != signature {
            bail!("signatures do not match");
        }

        Ok(WireMessage {
            header: msg_frames[0].clone(),
            parent_header: msg_frames[1].clone(),
            metadata: msg_frames[2].clone(),
            content: msg_frames[3].clone(),
            auth: auth.clone(),
        })
    }

    pub(crate) fn into_response(self) -> Result<Response> {
        let header_str = std::str::from_utf8(&self.header)?;
        let header: Header = serde_json::from_str(header_str)?;
        trace!("header: {:?}", header);

        let parent_header_str = std::str::from_utf8(&self.parent_header)?;
        let parent_header: Header = serde_json::from_str(parent_header_str)?;
        trace!("parent header: {:?}", parent_header);

        let metadata_str = std::str::from_utf8(&self.metadata)?;
        let metadata: Metadata = serde_json::from_str(metadata_str)?;
        trace!("metadata: {:?}", metadata);

        let content_str = std::str::from_utf8(&self.content)?;
        trace!("content string: {}", content_str);

        debug!("received message type `{}`", &header.msg_type);
        match header.msg_type.as_str() {
            "kernel_info_reply" => Ok(Response::Shell(ShellResponse::KernelInfo {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "execute_reply" => Ok(Response::Shell(ShellResponse::Execute {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "inspect_reply" => Ok(Response::Shell(ShellResponse::Inspect {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "complete_reply" => Ok(Response::Shell(ShellResponse::Complete {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "history_reply" => Ok(Response::Shell(ShellResponse::History {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "is_complete_reply" => {
                let content_json: Value = serde_json::from_str(content_str)?;
                let content = match content_json["status"] {
                    Value::String(ref s) if s == "complete" => IsCompleteStatus::Complete,
                    Value::String(ref s) if s == "invalid" => IsCompleteStatus::Invalid,
                    Value::String(ref s) if s == "unknown" => IsCompleteStatus::Unknown,
                    Value::String(ref s) if s == "incomplete" => {
                        let indent_node = &content_json["indent"];
                        let indent = String::from(
                            indent_node
                                .as_str()
                                .ok_or(format_err!("response indent value empty"))?,
                        );
                        IsCompleteStatus::Incomplete(indent)
                    }
                    _ => unreachable!(),
                };

                Ok(Response::Shell(ShellResponse::IsComplete {
                    header,
                    parent_header,
                    metadata,
                    content: content,
                }))
            }
            "shutdown_reply" => Ok(Response::Shell(ShellResponse::Shutdown {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "comm_info_reply" => Ok(Response::Shell(ShellResponse::CommInfo {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "status" => Ok(Response::IoPub(IoPubResponse::Status {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "execute_input" => Ok(Response::IoPub(IoPubResponse::ExecuteInput {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "stream" => Ok(Response::IoPub(IoPubResponse::Stream {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "error" => Ok(Response::IoPub(IoPubResponse::Error {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "execute_result" => Ok(Response::IoPub(IoPubResponse::ExecuteResult {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            "clear_output" => Ok(Response::IoPub(IoPubResponse::ClearOutput {
                header,
                parent_header,
                metadata,
                content: serde_json::from_str(content_str)?,
            })),
            _ => unreachable!("{}", header.msg_type),
        }
    }

    pub(crate) fn into_packets(self) -> Result<Vec<Part>> {
        let mut buf = Vec::with_capacity(4);

        // Start by adding the items that need a signature
        buf.push(self.header);
        buf.push(self.parent_header);
        buf.push(self.metadata);
        buf.push(self.content);

        let signature = sign(buf.as_slice(), self.auth.clone());

        let mut result = Vec::with_capacity(6);
        result.push(DELIMITER.to_vec());
        result.push(signature.into_bytes());
        result.extend_from_slice(&buf);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::Command;
    use crate::test_helpers::*;
    use serde_json::json;

    #[test]
    fn test_kernel_info_into_packets() {
        let command = Command::KernelInfo;
        assert_packets(PacketsTestData {
            command,
            expected_header_type: "kernel_info_request",
            expected_content: json!({}),
        });
    }

    #[test]
    fn test_execute_request_into_packets() {
        use std::collections::HashMap;

        let cmd = Command::Execute {
            code: "a = 10".to_string(),
            silent: false,
            store_history: true,
            user_expressions: HashMap::new(),
            allow_stdin: true,
            stop_on_error: false,
        };
        assert_packets(PacketsTestData {
            command: cmd,
            expected_header_type: "execute_request",
            expected_content: json!({
                "code": "a = 10",
                "silent": false,
                "store_history": true,
                "user_expressions": {},
                "allow_stdin": true,
                "stop_on_error": false,
            }),
        });
    }

    #[test]
    fn test_is_complete_into_packets() {
        let cmd = Command::IsComplete {
            code: "a = 10".to_string(),
        };
        assert_packets(PacketsTestData {
            command: cmd,
            expected_header_type: "is_complete_request",
            expected_content: json!({
                "code": "a = 10",
            }),
        });
    }

    #[test]
    fn test_shutdown_into_packets() {
        let cmd = Command::Shutdown { restart: false };
        assert_packets(PacketsTestData {
            command: cmd,
            expected_header_type: "shutdown_request",
            expected_content: json!({
                "restart": false,
            }),
        });
    }

    #[test]
    fn test_comm_info_packets() {
        let cmd = Command::CommInfo { target_name: None };
        assert_packets(PacketsTestData {
            command: cmd,
            expected_header_type: "comm_info_request",
            expected_content: json!({}),
        });
    }

    fn packets_from_command(command: Command) -> impl Iterator<Item = Part> {
        let auth = FakeAuth::create();
        let wire = command
            .into_wire(auth.clone())
            .expect("creating wire message");
        let packets = wire.into_packets().expect("creating packets");
        packets.into_iter()
    }

    fn check_packet_preamble(
        mut packets: impl Iterator<Item = Part>,
        expected_header_type: &str,
    ) -> impl Iterator<Item = Part> {
        use serde_json::json;

        let packet = packets.next().unwrap();
        compare_bytestrings!(&packet, &DELIMITER);

        let packet = packets.next().unwrap();
        compare_bytestrings!(&packet, &expected_signature().as_bytes());

        let packet = packets.next().unwrap();
        let header_str = std::str::from_utf8(&packet).unwrap();
        let header: Header = serde_json::from_str(header_str).unwrap();

        assert_eq!(header.msg_type, expected_header_type);

        // The rest of the packet should be empty maps
        let packet = packets.next().unwrap();
        let parent_header_str = std::str::from_utf8(&packet).unwrap();
        let parent_header: Value = serde_json::from_str(parent_header_str).unwrap();
        assert_eq!(parent_header, json!({}));

        let packet = packets.next().unwrap();
        let metadata_str = std::str::from_utf8(&packet).unwrap();
        let metadata: Value = serde_json::from_str(metadata_str).unwrap();
        assert_eq!(metadata, json!({}));
        packets
    }

    struct PacketsTestData {
        command: Command,
        expected_header_type: &'static str,
        expected_content: Value,
    }

    fn assert_packets(testdata: PacketsTestData) {
        let packets = packets_from_command(testdata.command);
        let mut packets = check_packet_preamble(packets, testdata.expected_header_type);

        // Check content
        let packet = packets.next().unwrap();
        let content_str = std::str::from_utf8(&packet).unwrap();
        let content: Value = serde_json::from_str(content_str).unwrap();
        assert_eq!(content, testdata.expected_content);
    }

}
