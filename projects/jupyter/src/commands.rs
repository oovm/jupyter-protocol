/*! Available commands to send to the kernel.

These should be constructed, and then sent to the kernel via
[`Client::send_shell_command`][send-shell-command] or
[`Client::send_control_command`][send-control-command].

[send-shell-command]: ../struct.Client.html#method.send_shell_command
[send-control-command]: ../struct.Client.html#method.send_control_command
*/
use crate::errors::Result;
use crate::header::Header;
use crate::wire::WireMessage;
use hmac::Mac;
use log::trace;
use serde::{Serialize as SerdeSerialize, Serializer};
use serde_derive::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Debug;

/** Available commands.
 */
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Command {
    /// Ask for information about the running kernel
    KernelInfo,
    /// Execute a specific command
    Execute {
        /// Source code to be executed by the kernel, one or more lines.
        code: String,
        /** A boolean flag which, if `true`, signals the kernel to execute
        this code as quietly as possible.  `silent=true` forces `store_history` to be `false`, and
        will *not*:
        - broadcast output on the IOPub channel
        - have an execute_result
        The default is `false`.
        */
        silent: bool,
        /** A boolean flag which, if `true`, signals the kernel to populate history
        The default is `true` if silent is `false`.  If silent is `true`, `store_history` is forced
        to be `false`.
        */
        store_history: bool,
        /** A `HashMap` mapping names to expressions to be evaluated in the
        user's `HashMap`. The rich display-data representation of each will be evaluated after
        execution.  See the `display_data` content for the structure of the representation data.
        */
        user_expressions: HashMap<String, String>,
        /** Some frontends do not support stdin requests.
        If this is true, code running in the kernel can prompt the user for input with an
        `input_request` message. If it is `false`, the kernel should not send these messages.
        */
        allow_stdin: bool,
        /** A boolean flag, which, if `true`, does not abort the execution queue, if an exception
         * is encountered.
        This allows the queued execution of multiple `execute_requests`, even if they generate
        exceptions.
        */
        stop_on_error: bool,
    },
    /// Perform introspection into a piece of code.
    Inspect {
        /** The code context in which introspection is requested
        this may be up to an entire multiline cell.
        */
        code: String,

        /** The cursor position within 'code' (in unicode characters) where inspection is requested
         */
        cursor_pos: u64,

        /** The level of detail desired.  In IPython, the default (0) is equivalent to typing
        'x?' at the prompt, 1 is equivalent to 'x??'.
        The difference is up to kernels, but in IPython level 1 includes the source code
        if available.
        */
        detail_level: DetailLevel,
    },
    /// Ask the kernel to complete the code at the cursor.
    Complete {
        /** The code context in which completion is requested
        this may be up to an entire multiline cell, such as
        'foo = a.isal'
        */
        code: String,

        /** The cursor position within 'code' (in unicode characters) where completion is requested
         */
        cursor_pos: u64,
    },
    /// Fetch history from the kernel.
    History {
        /// If True, also return output history in the resulting dict.
        output: bool,

        /// If True, return the raw input history, else the transformed input.
        raw: bool,

        /** So far, this can be `range`, `tail` or `search`.
        If `hist_access_type` is `range`, get a range of input cells. session
        is a number counting up each time the kernel starts; you can give
        a positive session number, or a negative number to count back from
        the current session.
        `start` and `stop` are line (cell) numbers within that session.
        If `hist_access_type` is 'tail' or 'search', get the last n cells.
        If `hist_access_type` is 'search', get cells matching the specified glob
        pattern (with * and ? as wildcards).
        */
        hist_access_type: HistoryAccessType,

        /** If `hist_access_type` is 'search' and unique is true, do not
        include duplicated history.  Default is false.
        */
        unique: bool,
    },
    /// Ask the kernel if the current code is complete
    IsComplete {
        /// The code entered so far as a multiline string
        code: String,
    },
    /// Tell the kernel to shutdown.
    Shutdown {
        /// False if final shutdown, or True if shutdown precedes a restart
        restart: bool,
    },
    /// Fetch comm info.
    CommInfo {
        /// The target name
        target_name: Option<String>,
    },
}

impl Command {
    pub(crate) fn into_wire<M: Mac + Debug>(self, auth: M) -> Result<WireMessage<M>> {
        let msg = match self {
            Command::KernelInfo => {
                let header = Header::new("kernel_info_request");
                let header_bytes = header.to_bytes()?;
                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content: b"{}".to_vec(),
                    auth,
                })
            }
            r @ Command::Execute { .. } => {
                let header = Header::new("execute_request");
                let header_bytes = header.to_bytes()?;
                let content_str = serde_json::to_string(&r)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
            r @ Command::Inspect { .. } => {
                let header = Header::new("inspect_request");
                let header_bytes = header.to_bytes()?;
                let content_str = serde_json::to_string(&r)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
            r @ Command::Complete { .. } => {
                let header = Header::new("complete_request");
                let header_bytes = header.to_bytes()?;
                let content_str = serde_json::to_string(&r)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
            Command::History {
                output,
                raw,
                hist_access_type,
                unique,
            } => {
                let header = Header::new("history_request");
                let header_bytes = header.to_bytes()?;

                let content = match hist_access_type {
                    HistoryAccessType::Tail { n } => json!({
                        "output": output,
                        "raw": raw,
                        "unique": unique,
                        "hist_access_type": "tail",
                        "session": null,
                        "start": null,
                        "stop": null,
                        "n": n,
                        "pattern": null,
                    }),
                    HistoryAccessType::Range {
                        session,
                        start,
                        stop,
                    } => json!({
                            "output": output,
                            "raw": raw,
                            "unique": unique,
                            "hist_access_type": "tail",
                            "session": session,
                            "start": start,
                            "stop": stop,
                            "n": null,
                            "pattern": null,
                    }),
                    HistoryAccessType::Search { pattern } => json!({
                            "output": output,
                            "raw": raw,
                            "unique": unique,
                            "hist_access_type": "tail",
                            "session": null,
                            "start": null,
                            "stop": null,
                            "n": null,
                            "pattern": pattern,
                    }),
                };

                let content_str = serde_json::to_string(&content)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
            Command::IsComplete { code } => {
                let header = Header::new("is_complete_request");
                let header_bytes = header.to_bytes()?;

                let content_json = json!({
                    "code": code,
                });
                let content_str = serde_json::to_string(&content_json)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content: content,
                    auth,
                })
            }
            Command::Shutdown { restart } => {
                let header = Header::new("shutdown_request");
                let header_bytes = header.to_bytes()?;
                let content_json = json!({
                    "restart": restart,
                });
                let content_str = serde_json::to_string(&content_json)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
            Command::CommInfo { target_name } => {
                let header = Header::new("comm_info_request");
                let header_bytes = header.to_bytes()?;
                let content_json = match target_name {
                    Some(target_name) => json!({
                        "target_name": target_name,
                    }),
                    None => json!({}),
                };
                let content_str = serde_json::to_string(&content_json)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
        };

        trace!("creating message {:?}", msg);
        msg
    }
}

/// Type of history requested.
#[derive(Serialize, Debug)]
pub enum HistoryAccessType {
    /// Get the last `n` cells.
    Tail {
        /// Number of cells requested.
        n: u64,
    },
    /// Get the range of cells associated with a session within a range.
    Range {
        /// Session to query for.
        session: i64,
        /// Start of the range
        start: u64,
        /// End of the range.
        stop: u64,
    },
    /// Search for history items matching a pattern.
    Search {
        /// Pattern to search for.
        pattern: String,
    },
}

/// Level of detail requested when requesting code introspection
#[derive(Debug)]
pub enum DetailLevel {
    /** Equivalent to IPython's `?` operator.

    Typically fetches the documentation.
    */
    Zero,
    /** Equivalent to IPython's `??` operator.

    Typically fetches the source code.
    */
    One,
}

impl SerdeSerialize for DetailLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DetailLevel::Zero => serializer.serialize_i32(0),
            DetailLevel::One => serializer.serialize_i32(1),
        }
    }
}
