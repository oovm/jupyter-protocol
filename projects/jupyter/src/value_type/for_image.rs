use crate::{Executed, JupyterTheme};
use image::{codecs::png::PngEncoder, ColorType, DynamicImage, ImageEncoder};
use serde_json::Value;
use std::fmt::Formatter;

impl Executed for DynamicImage {
    fn mime_type(&self) -> String {
        "image/png".to_string()
    }

    fn as_json(&self, theme: JupyterTheme) -> Value {
        let bg = match theme {
            JupyterTheme::Light => image::Rgba([255, 255, 255, 255]),
            JupyterTheme::Dark => image::Rgba([0, 0, 0, 255]),
        };
        let mut buf = Vec::new();
        let mut writer = PngEncoder::new(&mut buf);
        writer.write_image(self.to_rgb8().as_raw(), self.width(), self.height(), ColorType::Rgba8).unwrap();
        let data = base64::encode(&buf);
        let data_url = format!("data:image/png;base64,{}", data);
        Value::String(format!(
            r#"<img src="{}" style="background-color:#{:02x}{:02x}{:02x};" />"#,
            data_url, bg[0], bg[1], bg[2]
        ))
    }
}
