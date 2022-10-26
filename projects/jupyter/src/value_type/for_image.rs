use crate::{Executed, JupyterTheme};
use image::{codecs::png::PngEncoder, ColorType, DynamicImage, ImageEncoder, RgbaImage};
use serde_json::Value;

impl Executed for RgbaImage {
    fn mime_type(&self) -> String {
        "image/png".to_string()
    }

    #[allow(deprecated)]
    fn as_json(&self, _: JupyterTheme) -> Value {
        let mut buf = Vec::new();
        let writer = PngEncoder::new(&mut buf);
        writer.write_image(self.as_raw(), self.width(), self.height(), ColorType::Rgba8).unwrap();
        let data = base64::encode(&buf);
        let data_url = format!("data:image/png;base64,{}", data);
        Value::String(format!(r#"<img src="{}"/>"#, data_url,))
    }
}

impl Executed for DynamicImage {
    fn mime_type(&self) -> String {
        "image/png".to_string()
    }

    fn as_json(&self, theme: JupyterTheme) -> Value {
        self.to_rgba8().as_json(theme)
    }
}
