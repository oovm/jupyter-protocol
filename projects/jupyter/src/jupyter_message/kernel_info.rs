use super::*;
use crate::LanguageInfo;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KernelInfoReply {
    status: String,
    protocol_version: String,
    implementation: String,
    implementation_version: String,
    language_info: SealedLanguageInfo,
    debugger: bool,
    banner: String,
    help_links: Vec<HelpLink>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct SealedLanguageInfo {
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

impl JupiterContent {
    pub fn build_kernel_info(info: LanguageInfo) -> JupiterContent {
        let content = KernelInfoReply::build(info);
        JupiterContent::KernelInfoReply(Box::new(content))
    }
}

impl KernelInfoReply {
    /// See [Kernel info documentation](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-info)
    pub fn build(info: LanguageInfo) -> KernelInfoReply {
        KernelInfoReply {
            status: "ok".to_owned(),
            protocol_version: "5.3".to_owned(),
            implementation: env!("CARGO_PKG_NAME").to_owned(),
            implementation_version: env!("CARGO_PKG_VERSION").to_owned(),
            language_info: SealedLanguageInfo {
                name: info.language,
                version: info.version,
                mimetype: "text/rust".to_owned(),
                file_extension: info.file_extensions,
                pygment_lexer: info.lexer,
                codemirror_mode: info.highlighter,
                nbconvert_exporter: info.exporter,
            },
            debugger: true,
            banner: format!("Jupyter Server Protocol v{} in Rust", env!("CARGO_PKG_VERSION")),
            help_links: vec![HelpLink {
                text: "Rust std docs".to_owned(),
                url: "https://doc.rust-lang.org/std/index.html".to_owned(),
            }],
        }
    }
}
