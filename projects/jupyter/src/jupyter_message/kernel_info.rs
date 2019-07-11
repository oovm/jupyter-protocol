use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct SealKernelInfo {
    protocol_version: String,
    implementation: String,
    implementation_version: String,
    language_info: SealLanguageInfo,
    banner: String,
    help_links: Vec<HelpLink>,
    status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct SealLanguageInfo {
    name: String,
    version: String,
    mimetype: String,
    file_extension: String,
    // Pygments lexer, for highlighting Only needed if it differs from the 'name' field.
    // see http://pygments.org/docs/lexers/#lexers-for-the-rust-language
    pygment_lexer: String,
    // Codemirror mode, for for highlighting in the notebook. Only needed if it differs from the 'name' field.
    // codemirror use text/x-rustsrc as mimetypes
    // see https://codemirror.net/mode/rust/
    codemirror_mode: String,
    nbconvert_exporter: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HelpLink {
    text: String,
    url: String,
}

impl JupyterMessageType {
    pub fn build_kernel_info_reply<T>(context: &T) -> JupyterMessage
    where
        T: ExecuteContext,
    {
        let header = JupyterMessageHeader::new(JupyterMessageType::KernelInfoReply);
        let content = SealKernelInfo::build(context);
        JupyterMessage::new(header, content)
    }
}

impl SealKernelInfo {
    /// See [Kernel info documentation](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-info)
    pub fn build<T>(context: &T) -> SealKernelInfo
    where
        T: ExecuteContext,
    {
        let language = context.language_info();
        SealKernelInfo {
            status: "ok".to_owned(),
            protocol_version: "5.3".to_owned(),
            implementation: env!("CARGO_PKG_NAME").to_owned(),
            implementation_version: env!("CARGO_PKG_VERSION").to_owned(),
            language_info: SealLanguageInfo {
                name: language.language,
                version: "".to_owned(),
                mimetype: "text/rust".to_owned(),
                file_extension: language.file_extensions,
                pygment_lexer: "rust".to_owned(),
                codemirror_mode: "rust".to_owned(),
                nbconvert_exporter: "rust".to_owned(),
            },
            banner: format!("EvCxR {} - Evaluation Context for Rust", env!("CARGO_PKG_VERSION")),
            help_links: vec![HelpLink {
                text: "Rust std docs".to_owned(),
                url: "https://doc.rust-lang.org/std/index.html".to_owned(),
            }],
        }
    }
}
