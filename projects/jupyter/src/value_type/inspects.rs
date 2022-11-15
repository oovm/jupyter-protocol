use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InspectVariable {
    pub name: String,
    pub value: String,
    #[serde(rename = "type")]
    pub typing: String,
    #[serde(rename = "evaluateName")]
    pub evaluate_name: String,
    #[serde(rename = "variablesReference")]
    pub variables_reference: i64,
}
#[derive(Clone, Debug, Serialize)]
pub struct InspectModule {
    pub id: u32,
    pub name: String,
    pub path: String,
}
impl Default for InspectVariable {
    fn default() -> Self {
        Self {
            name: "undefined".to_string(),
            value: "any".to_string(),
            typing: "Any".to_string(),
            evaluate_name: "".to_string(),
            variables_reference: 0,
        }
    }
}

impl InspectVariable {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into(), ..Self::default() }
    }
    pub fn with_value<T, V>(self, typing: T, value: V) -> Self
    where
        T: Into<String>,
        V: Into<String>,
    {
        Self { value: value.into(), typing: typing.into(), ..self }
    }
}
