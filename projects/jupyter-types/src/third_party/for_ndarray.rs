use crate::{value_type::JupyterContext, Executed};
use ndarray::{Array1, Array2};
use serde_json::Value;
use std::fmt::Display;

impl<T> Executed for Array1<T>
where
    T: Display + Send,
{
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        let mut html = "<table><tbody>".to_string();
        for i in self.iter() {
            html.push_str(&format!("<tr><td>{}</td></tr>", i));
        }
        html.push_str("</tbody></table>");
        Value::String(html)
    }
}

impl<T> Executed for Array2<T>
where
    T: Display + Send,
{
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: &JupyterContext) -> Value {
        let mut html = "<table><tbody>".to_string();
        for i in self.rows() {
            html.push_str("<tr>");
            for j in i.iter() {
                html.push_str(&format!("<td>{}</td>", j));
            }
            html.push_str("</tr>");
        }
        html.push_str("</tbody></table>");
        Value::String(html)
    }
}
