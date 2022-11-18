use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// An identifier for a variable.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InspectVariable {
    ///  The name of the variable.
    pub name: String,
    ///  The value of the variable.
    pub value: String,
    ///   The type of the variable.
    #[serde(rename = "type")]
    pub typing: String,
    ///   The variables's evaluation kind.
    #[serde(rename = "evaluateName")]
    pub evaluate_name: String,
    ///    The variables's evaluation kind.
    #[serde(rename = "variablesReference")]
    pub variables_reference: usize,
}

/// An identifier for a module.
#[derive(Clone, Debug, Serialize)]
pub struct InspectModule {
    ///   The module's identifier.
    pub id: u32,
    ///   The module's name.
    pub name: String,
    ///   The module's path.
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
    /// Create a new variable.
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into(), ..Self::default() }
    }
    ///  Create a new variable with a value.
    pub fn with_value<T, V>(self, typing: T, value: V) -> Self
    where
        T: Into<String>,
        V: Into<String>,
    {
        Self { value: value.into(), typing: typing.into(), ..self }
    }
}
