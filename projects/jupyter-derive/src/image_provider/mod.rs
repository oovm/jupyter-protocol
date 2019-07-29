pub struct PngProvider {
    path: Option<LitStr>,
}

impl Parse for PngProvider {
    fn parse(input: ParseStream) -> Result<Self> {
        let path = if input.peek(LitStr) {
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self {
            path,
        })
    }
}

impl ToTokens for PngProvider {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = self.path.as_ref().map(|p| p.value());
        let path = path.as_ref().map(|p| p.as_str()).unwrap_or("default.png");
        let path = format!("{}{}", "assets/", path);
        let path = LitStr::new(&path, Span::call_site());
        let path = quote! { #path };
    }
}