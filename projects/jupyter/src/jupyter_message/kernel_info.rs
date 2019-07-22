use super::*;
use crate::{ExecuteContext, LanguageInfo};


#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KernelInfo {
    protocol_version: String,
    implementation: String,
    implementation_version: String,
    language_info: SealedLanguageInfo,
    banner: String,
    help_links: Vec<HelpLink>,
    status: String,
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
        let content = KernelInfo::build(info);
        JupiterContent::KernelInfo(Box::new(content))
    }
}

impl KernelInfo {
    /// See [Kernel info documentation](https://jupyter-client.readthedocs.io/en/stable/messaging.html#kernel-info)
    pub fn build(info: LanguageInfo) -> KernelInfo {
        KernelInfo {
            status: "ok".to_owned(),
            protocol_version: "5.3".to_owned(),
            implementation: env!("CARGO_PKG_NAME").to_owned(),
            implementation_version: env!("CARGO_PKG_VERSION").to_owned(),
            language_info: SealedLanguageInfo {
                name: info.language,
                version: "".to_owned(),
                mimetype: "text/rust".to_owned(),
                file_extension: info.file_extensions,
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
