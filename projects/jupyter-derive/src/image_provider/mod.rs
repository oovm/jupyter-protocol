use quote::{ToTokens, __private::TokenStream};
use syn::{
    parse::{Parse, ParseStream},
    LitStr,
};

use image::{
    codecs::png::PngDecoder, imageops::FilterType, io::Reader, GenericImage, ImageDecoder, ImageFormat, ImageResult, RgbaImage,
};
use std::io::Cursor;

pub fn bytes_to_png(bytes: &[u8]) -> ImageResult<RgbaImage> {
    let mut reader = Reader::new(Cursor::new(bytes));
    reader.set_format(ImageFormat::Png);
    Ok(reader.decode()?.resize_exact(64, 64, FilterType::Lanczos3).to_rgba8())
}

pub fn png_to_bytes(image: &RgbaImage, size: u32) -> ImageResult<Vec<u8>> {
    // exactly resize the image
    let resized = image.res(size, size, FilterType::Lanczos3);

    let mut buffer = Vec::new();
    let mut encoder = image::codecs::png::PngEncoder::new(&mut buffer);
}

pub struct LogoProvider {
    path: Option<LitStr>,
}

impl Parse for LogoProvider {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<LitStr>().map(|path| LogoProvider { path: Some(path) })
    }
}

impl ToTokens for LogoProvider {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        panic!("{}", self.path);
        todo!()
    }
}
