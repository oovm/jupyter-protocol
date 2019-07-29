mod image_provider;



#[proc_macro]
pub fn include_png64(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as image_provider::ImageProvider);
    let output = image_provider::expand_image_provider(input);
    TokenStream::from(output)
}

#[proc_macro]
pub fn include_png32(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as image_provider::ImageProvider);
    let output = image_provider::expand_image_provider(input);
    TokenStream::from(output)
}

/// include_png32;
/// include_png64;
