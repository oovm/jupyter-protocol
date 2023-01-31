#![allow(dead_code)]

//! <https://github.com/gnestor/notebook/blob/master/notebook/static/notebook/js/outputarea.js#L260>
use image::RgbaImage;
use jupyter::third_party::{Array1, Array2, MathML, Url};
use mathml_core::MathRoot;
use mathml_latex::{parse_latex, LaTeXEngine};
use std::str::FromStr;
use svg::{
    node::element::{path::Data, Path},
    Document,
};

pub fn test_svg() -> Document {
    let data = Data::new().move_to((10, 10)).line_by((0, 50)).line_by((50, 0)).line_by((0, -50)).close();
    let path = Path::new().set("fill", "none").set("stroke", "black").set("stroke-width", 3).set("d", data);
    Document::new().set("viewBox", (0, 0, 70, 70)).add(path)
}

pub fn test_mathml() -> MathML {
    let context = LaTeXEngine::builtin();
    let math = parse_latex(r#"a + \dfrac{1}{b + \dfrac{1}{c + \dfrac{1}{d + \dfrac{1}{e}}}}"#).expect("invalid tex");
    MathRoot::new(vec![math.as_mathml(&context)].into_iter()).with_namespace().into()
}

pub fn test_url() -> Url {
    Url::from_str("https://github.com/oovm/jupyter-protocol").expect("invalid url")
}

pub fn test_json() -> jupyter::Value {
    toml::from_str(include_str!("../../Cargo.toml")).expect("invalid toml")
}

pub fn test_png() -> RgbaImage {
    let data = include_bytes!("../../third_party/rust/rust-logo-32x32.png");
    image::load_from_memory(data).unwrap().to_rgba8()
}

pub fn test_array1() -> Array1<f64> {
    let mut array = Array1::<f64>::zeros(10);
    for i in 0..10 {
        array[i] = i as f64;
    }
    array
}

pub fn test_array2() -> Array2<f64> {
    let mut array = Array2::<f64>::zeros((10, 10));
    for i in 0..10 {
        for j in 0..10 {
            array[[i, j]] = i as f64 + j as f64;
        }
    }
    array
}
