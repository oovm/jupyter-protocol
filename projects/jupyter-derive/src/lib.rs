extern crate proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod image_provider;

use crate::image_provider::LogoProvider;

#[proc_macro]
pub fn include_png64(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LogoProvider);
    input.with_size(64).to_token_stream().into()
}

#[proc_macro]
pub fn include_png32(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LogoProvider);
    input.with_size(32).to_token_stream().into()
}

// #[proc_macro]
// pub fn include_png32(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as image_provider::ImageProvider);
//     let output = image_provider::expand_image_provider(input);
//     TokenStream::from(output)
// }
