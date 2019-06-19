/*! Available responses back from the kernel.
*/
use crate::header::Header;
use crate::metadata::Metadata;
use serde_derive::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// Link pointing to some help text.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct HelpLink {
    /// The text to display.
    pub text: String,
    /// The url to point to.
    pub url: String,
}

/** Overall response type

There are two responses available:

- responses that come from sending a shell message, and
- responses that come from the IOPub socket.

These two responses are then wrapped into a single `Response` type so that functions can return any
response.
*/
#[derive(Debug)]
pub enum Response {
    /// Response from sending a shell message.
    Shell(ShellResponse),
    /// Response from the IOPub socket, sent from the kernel.
    IoPub(IoPubResponse),
}

/// Responses from sending shell messages.
#[derive(Debug)]
pub enum ShellResponse {
    /// Response from asking for information about the running kernel.
    KernelInfo {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: KernelInfoContent,
    },
    /// Response from sending an execute request.
    Execute {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: ExecuteReplyContent,
    },
    /// Response from inspecting a code block.
    Inspect {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: InspectContent,
    },
    /// Resposne from asking for code completion.
    Complete {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: CompleteContent,
    },
    /// Response from fetching kernel command history.
    History {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: HistoryContent,
    },
    /// Response from asking the kernel if the code is complete.
    IsComplete {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: IsCompleteStatus,
    },
    /// Response from asking to shut down the kernel.
    Shutdown {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: ShutdownContent,
    },
    /// Response from asking information about comms.
    CommInfo {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: CommInfoContent,
    },
}

/// Responses from the IOPub channel.
#[derive(Debug)]
pub enum IoPubResponse {
    /// Response from the kernel showing the current kernel status.
    Status {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: StatusContent,
    },
    /// Response when any code is run so all clients are aware of it.
    ExecuteInput {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: ExecuteInputContent,
    },
    /// Response when something is written to stdout/stderr.
    Stream {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: StreamContent,
    },
    /// Response when a response has mutliple formats.
    ExecuteResult {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: ExecuteResultContent,
    },
    /// Response when an error occurs.
    Error {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: ErrorContent,
    },
    /// Response when the kernel askes the client to clear it's output.
    ClearOutput {
        /// Header from the kernel.
        header: Header,
        /// Header sent to the kernel.
        parent_header: Header,
        /// Metadata about the response.
        metadata: Metadata,
        /// Main response content.
        content: ClearOutputContent,
    },
}

/// Content for a KernelInfo response.
#[derive(Deserialize, Debug)]
pub struct KernelInfoContent {
    /// Status of the request.
    pub status: Status,
    /// Version of the messaging protocol.
    pub protocol_version: String,
    /// The kernel implementation name.
    pub implementation: String,
    /// The kernel implementation version.
    pub implementation_version: String,
    /// Information about the language of code for the kernel.
    pub language_info: LanguageInfo,
    /// A banner of information about the kernel.
    pub banner: String,
    /// List of help entries.
    pub help_links: Vec<HelpLink>,
}

/// Information about the language of code for the kernel.
#[derive(Deserialize, Debug)]
pub struct LanguageInfo {
    /// Name of the programming language the kernel implements.
    pub name: String,
    /// The language version number.
    pub version: String,
    /// Mimetype for script files in this language.
    pub mimetype: String,
    /// Extension including the dot e.g. '.py'
    pub file_extension: String,
    /// Pygments lexer for highlighting.
    pub pygments_lexer: String,
    /// Codemirror mode, for highlighting in the notebook.
    pub codemirror_mode: Value,
    /// If notebooks written with this kernel should be exported with something other than the
    /// general 'script' exporter.
    pub nbconvert_exporter: String,
}

/// Information from code execution.
#[derive(Deserialize, Debug)]
pub struct ExecuteReplyContent {
    /// Status of the request.
    pub status: Status,
    /// Global execution count.
    pub execution_count: i64,
    // status == "ok" fields
    /// List of payload dicts (deprecated).
    pub payload: Option<Vec<HashMap<String, Value>>>,
    /// Results for the user expressions.
    pub user_expressions: Option<HashMap<String, Value>>,
    // status == "error" fields
    /// Exception name as a string.
    pub ename: Option<String>,
    /// Exception value, as a string.
    pub evalue: Option<String>,
    /// Traceback frames as strings.
    pub traceback: Option<Vec<String>>,
}

/// Response from the IOPub status messages
#[derive(Deserialize, Debug)]
pub struct StatusContent {
    /// The state of the kernel.
    pub execution_state: ExecutionState,
}

/// Response when code is input to the kernel.
#[derive(Deserialize, Debug)]
pub struct ExecuteInputContent {
    /// The code that was run.
    pub code: String,
    /// Counter for the execution number.
    pub execution_count: i64,
}

/// Response from inspecting code
#[derive(Deserialize, Debug)]
pub struct InspectContent {
    /// Status of the request.
    pub status: Status,
    /// Whether the object was found or not.
    pub found: bool,
    /// Empty if nothing is found.
    pub data: HashMap<String, Value>,
    /// Metadata.
    pub metadata: HashMap<String, Value>,
}

/// Response when printing to stdout/stderr.
#[derive(Deserialize, Debug)]
pub struct StreamContent {
    /// Type of the stream.
    pub name: StreamType,
    /// Text to be written to the stream.
    pub text: String,
}

/// Content of an error response.
#[derive(Deserialize, Debug)]
pub struct ErrorContent {
    /// Exception name as a string.
    pub ename: String,
    /// Exception value, as a string.
    pub evalue: String,
    /// Traceback frames as strings.
    pub traceback: Vec<String>,
}

/// Content when asking for code completion.
#[derive(Deserialize, Debug)]
pub struct CompleteContent {
    /// Status of the request.
    pub status: Status,
    /// List of all matches.
    pub matches: Vec<String>,
    /// The start index text that should be replaced by the match.
    pub cursor_start: u64,
    /// The end index text that should be replaced by the match.
    pub cursor_end: u64,
    /// Extra information.
    pub metadata: HashMap<String, Value>,
}

/// Content when asking for history entries.
#[derive(Deserialize, Debug)]
pub struct HistoryContent {
    /// Status of the request.
    pub status: Status,
    /// List of history items.
    pub history: Vec<Value>,
}

/// Response when asking the kernel to shutdown.
#[derive(Deserialize, Debug)]
pub struct ShutdownContent {
    /// Status of the request.
    pub status: Status,
    /// Whether restart was requested.
    pub restart: bool,
}

/// Response when asking for comm info.
#[derive(Deserialize, Debug)]
pub struct CommInfoContent {
    /// Status of the request.
    pub status: Status,
    /// Map of available comms.
    pub comms: HashMap<String, HashMap<String, String>>,
}

/// Response when requesting to execute code.
#[derive(Deserialize, Debug)]
pub struct ExecuteResultContent {
    /// Global execution count.
    pub execution_count: i64,
    /// The result of the execution.
    pub data: HashMap<String, String>,
    /// Metadata about the execution.
    pub metadata: Value,
}

/// Response when the kernel asks the client to clear the output.
#[derive(Deserialize, Debug)]
pub struct ClearOutputContent {
    /// Wait to clear the output until new output is available.
    pub wait: bool,
}

/// State of the kernel.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionState {
    /// Running code.
    Busy,
    /// Doing nothing.
    Idle,
    /// Booting.
    Starting,
}

/// Status of if entered code is complete (i.e. does not need another " character).
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IsCompleteStatus {
    /// Entered code is complete.
    Complete,
    /// More code is required. The argument is the indent value for the prompt.
    Incomplete(String),
    /// Invalid completion.
    Invalid,
    /// Unknown completion.
    Unknown,
}

/// Type of stream, either stdout or stderr.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum StreamType {
    Stdout,
    Stderr,
}

/// Status of the request.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum Status {
    Ok,
    Error,
    Abort,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use crate::wire::WireMessage;

    #[test]
    fn test_kernel_info_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "kernel_info_reply",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "kernel_info_request",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "banner": "banner",
                "implementation": "implementation",
                "implementation_version": "implementation_version",
                "protocol_version": "protocol_version",
                "status": "ok",
                "language_info": {
                    "name": "python",
                    "version": "3.7.0",
                    "mimetype": "text/x-python",
                    "file_extension": ".py",
                    "pygments_lexer": "ipython3",
                    "codemirror_mode": {
                        "name": "ipython",
                        "version": 3
                    },
                    "nbconvert_exporter": "python"
                },
                "help_links": [{"text": "text", "url": "url"}]
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::KernelInfo {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "kernel_info_reply");

                // Check the content
                assert_eq!(content.banner, "banner");
                assert_eq!(content.implementation, "implementation");
                assert_eq!(content.implementation_version, "implementation_version");
                assert_eq!(content.protocol_version, "protocol_version");
                assert_eq!(content.status, Status::Ok);
                assert_eq!(
                    content.help_links,
                    vec![HelpLink {
                        text: "text".to_string(),
                        url: "url".to_string(),
                    }]
                );
                assert_eq!(content.language_info.name, "python");
            }
            _ => unreachable!("Incorrect response type, should be KernelInfo"),
        }
    }

    #[test]
    fn test_execute_request_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_reply",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_request",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "execution_count": 4
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::Execute {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "execute_reply");

                // Check the content
                assert_eq!(content.status, Status::Ok);
                assert_eq!(content.execution_count, 4);
            }
            _ => unreachable!("Incorrect response type, should be KernelInfo"),
        }
    }

    #[test]
    fn test_status_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "status",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header, not relevant
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_request",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "execution_state": "busy"
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::IoPub(IoPubResponse::Status {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "status");

                // Check the content
                assert_eq!(content.execution_state, ExecutionState::Busy);
            }
            _ => unreachable!("Incorrect response type, should be Status"),
        }
    }

    #[test]
    fn test_execute_input_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_input",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header, not relevant
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "code": "a = 10",
                "execution_count": 4
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::IoPub(IoPubResponse::ExecuteInput {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "execute_input");

                // Check the content
                assert_eq!(content.code, "a = 10");
                assert_eq!(content.execution_count, 4);
            }
            _ => unreachable!("Incorrect response type, should be Status"),
        }
    }

    #[test]
    fn test_stream_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "stream",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header, not relevant
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "name": "stdout",
                "text": "10"
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::IoPub(IoPubResponse::Stream {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "stream");

                // Check the content
                assert_eq!(content.name, StreamType::Stdout);
                assert_eq!(content.text, "10");
            }
            _ => unreachable!("Incorrect response type, should be Stream"),
        }
    }

    #[test]
    fn test_is_complete_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "is_complete_reply",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "is_complete_request",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "complete"
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::IsComplete {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "is_complete_reply");

                // Check the content
                assert_eq!(content, IsCompleteStatus::Complete);
            }
            _ => unreachable!("Incorrect response type, should be IsComplete"),
        }
    }

    #[test]
    fn test_is_complete_message_parsing_with_incomplete_reply() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "is_complete_reply",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "is_complete_request",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "incomplete",
                "indent": "  "
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::IsComplete {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "is_complete_reply");

                // Check the content
                assert_eq!(content, IsCompleteStatus::Incomplete("  ".to_string()));
            }
            _ => unreachable!("Incorrect response type, should be IsComplete"),
        }
    }

    #[test]
    fn test_shutdown_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "shutdown_reply",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "kernel_info_request",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "restart": false
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::Shutdown {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "shutdown_reply");

                // Check the content
                assert_eq!(content.restart, false);
            }
            _ => unreachable!("Incorrect response type, should be KernelInfo"),
        }
    }

    #[test]
    fn test_comm_info_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "comm_info_reply",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "comm_info_request",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "status": "ok",
                "comms": {
                    "u-u-i-d": {
                        "target_name": "foobar"
                    }
                }
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::Shell(ShellResponse::CommInfo {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "comm_info_reply");

                // Check the content
                assert_eq!(content.comms["u-u-i-d"]["target_name"], "foobar");
            }
            _ => unreachable!("Incorrect response type, should be CommInfo"),
        }
    }

    #[test]
    fn test_execute_result_message_parsing() {
        use serde_json::json;

        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_result",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_request",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "data": {
                    "text/plain": "10"
                },
                "metadata": {},
                "execution_count": 46
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::IoPub(IoPubResponse::ExecuteResult {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "execute_result");

                // Check the content
                assert_eq!(content.data["text/plain"], "10");
                assert_eq!(content.metadata, json!({}));
                assert_eq!(content.execution_count, 46);
            }
            _ => unreachable!("Incorrect response type, should be ExecuteResult"),
        }
    }

    #[test]
    fn test_clear_output_message_parsing() {
        let auth = FakeAuth::create();
        let raw_response = vec![
            "<IDS|MSG>".to_string().into_bytes(),
            expected_signature().into_bytes(),
            // Header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "clear_output",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Parent header
            r#"{
                "date": "",
                "msg_id": "",
                "username": "",
                "session": "",
                "msg_type": "execute_request",
                "version": ""
            }"#
            .to_string()
            .into_bytes(),
            // Metadata
            r#"{}"#.to_string().into_bytes(),
            // Content
            r#"{
                "wait": false
            }"#
            .to_string()
            .into_bytes(),
        ];
        let msg = WireMessage::from_raw_response(raw_response, auth.clone()).unwrap();
        let response = msg.into_response().unwrap();
        match response {
            Response::IoPub(IoPubResponse::ClearOutput {
                header,
                parent_header: _parent_header,
                metadata: _metadata,
                content,
            }) => {
                // Check the header
                assert_eq!(header.msg_type, "clear_output");

                // Check the content
                assert_eq!(content.wait, false);
            }
            _ => unreachable!("Incorrect response type, should be ClearOutput"),
        }
    }
}
