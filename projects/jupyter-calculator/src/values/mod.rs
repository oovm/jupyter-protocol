#![allow(dead_code)]

//! <https://github.com/gnestor/notebook/blob/master/notebook/static/notebook/js/outputarea.js#L260>
use jupyter::value_type::{MathML, Url};
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
    MathRoot::new(vec![math.as_mathml(&context)]).with_namespace().into()
}

pub fn test_url() -> Url {
    Url::from_str("https://github.com/oovm/jupyter-protocol").expect("invalid url")
}

pub fn test_json() -> jupyter::Value {
    toml::from_str(include_str!("../../Cargo.toml")).expect("invalid toml")
}
