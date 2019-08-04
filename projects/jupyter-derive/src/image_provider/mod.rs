use quote::{ToTokens, __private::TokenStream, quote};
use syn::{
    parse::{Parse, ParseStream},
    LitStr,
};

use image::{
    codecs::png::{PngDecoder, PngEncoder},
    imageops::FilterType,
    io::Reader,
    ColorType, GenericImage, ImageDecoder, ImageEncoder, ImageFormat, ImageResult, RgbaImage,
};
use proc_macro2::Span;
use std::{io::Cursor, path::Path};

const DEFAULT_LOGO: &[u8] = include_bytes!("rust-logo.png");

pub fn bytes_to_png(bytes: &[u8], size: u32) -> ImageResult<RgbaImage> {
    let mut reader = Reader::new(Cursor::new(bytes));
    reader.set_format(ImageFormat::Png);
    Ok(reader.decode()?.resize_exact(size, size, FilterType::Lanczos3).to_rgba8())
}

pub fn png_to_bytes(image: &RgbaImage) -> ImageResult<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut encoder = PngEncoder::new(&mut bytes);
    encoder.write_image(image.as_raw(), image.width(), image.height(), ColorType::Rgba8)?;
    Ok(bytes)
}

pub struct LogoProvider {
    size: u32,
    path: Option<LitStr>,
}

impl Parse for LogoProvider {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<LitStr>().map(|path| LogoProvider { size: 64, path: Some(path) })
    }
}

impl ToTokens for LogoProvider {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let slice = match &self.path {
            None => DEFAULT_LOGO.to_vec(),
            Some(s) => self.load_image(s).unwrap(),
        };
        tokens.extend(quote! {
            &[#(#slice),*]
        });
    }
}

impl LogoProvider {
    pub fn with_size(self, size: u32) -> Self {
        Self { size, path: self.path }
    }
    pub fn load_image(&self, path: &LitStr) -> ImageResult<Vec<u8>> {
        // let file = Span::call_site().source_file();
        let value = path.value();
        let file = Path::new(&value);
        let dir = Path::new("./").canonicalize()?;
        if !file.exists() {
            panic!("file {} not found in: {}", file.display(), dir.display());
        }
        let png = bytes_to_png(&std::fs::read(&path.value())?, self.size)?;
        png_to_bytes(&png)
    }
}
