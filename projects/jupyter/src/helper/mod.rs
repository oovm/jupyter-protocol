use std::io::Cursor;
use image::codecs::png::PngDecoder;
use image::{GenericImage, ImageDecoder, ImageFormat, ImageResult, RgbaImage};
use image::imageops::FilterType;
use image::io::Reader;

pub fn bytes_to_png(bytes: &[u8]) -> ImageResult<RgbaImage> {
    let mut reader = Reader::new(Cursor::new(bytes));
    reader.set_format(ImageFormat::Png);
    Ok(reader.decode()?.resize_exact(64, 64, FilterType::Lanczos3).to_rgba8())
}

pub fn png_to_bytes(image:&RgbaImage, size: u32) -> ImageResult<Vec<u8>> {
    //exactly resize the image
    let resized = image.res(size, size, FilterType::Lanczos3);

    let mut buffer = Vec::new();
    let mut encoder = image::codecs::png::PngEncoder::new(&mut buffer);


}