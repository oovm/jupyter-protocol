use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{
    any::Any,
    fmt::{Debug, Formatter},
    num::NonZeroUsize,
};
use uuid::Uuid;

mod der;

pub struct InspectVariableRequest {
    /// The variable for which to retrieve its children. The `variablesReference`
    /// must have been obtained in the current suspended state. See 'Lifetime of
    /// Object References' in the Overview section for details.
    pub id: NonZeroUsize,
    /// Filter to limit the child variables to either named or indexed. If omitted,
    /// both types are fetched.
    /// Values: 'indexed', 'named'
    pub filter: Option<InspectVariableFilter>,
    /// The index of the first variable to return; if omitted children start at 0.
    /// The attribute is only honored by a debug adapter if the corresponding
    /// capability `supportsVariablePaging` is true.
    pub start: usize,
    /// The number of variables to return. If count is missing or 0, all variables
    /// are returned.
    /// The attribute is only honored by a debug adapter if the corresponding
    /// capability `supportsVariablePaging` is true.
    pub limit: Option<NonZeroUsize>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InspectVariableFilter {
    Indexed,
    Named,
}

/// A Variable is a name/value pair.
///
/// The type attribute is shown if space permits or when hovering over the variable’s name.
///
/// The kind attribute is used to render additional properties of the variable, e.g. different icons can be used to indicate that a variable is public or private.
///
/// If the value is structured (has children), a handle is provided to retrieve the children with the variables request.
///
/// If the number of named or indexed children is large, the numbers should be returned via the namedVariables and indexedVariables attributes.
///
/// The client can use this information to present the children in a paged UI and fetch them in chunks.
#[derive(Clone, Debug, Serialize)]
pub struct InspectVariable {
    /// The identifier of the variable. If it is empty, then the reference to this variable will not be sent later.
    #[serde(rename = "variablesReference")]
    pub id: Option<NonZeroUsize>,
    /// The variable's name.
    pub name: String,
    /// The variable's value.
    /// This can be a multi-line text, e.g. for a function the body of a function.
    /// For structured variables (which do not have a simple value), it is
    /// recommended to provide a one-line representation of the structured object.
    /// This helps to identify the structured object in the collapsed state when
    /// its children are not yet visible.
    /// An empty string can be used if no value should be shown in the UI.
    pub value: String,
    /// The type of the variable's value. Typically shown in the UI when hovering
    /// over the value.
    /// This attribute should only be returned by a debug adapter if the
    /// corresponding capability `supportsVariableType` is true.
    #[serde(rename = "type")]
    pub typing: String,
    /// The evaluate name of this variable which can be passed to the `evaluate`
    /// request to fetch the variable's value.
    #[serde(rename = "evaluateName")]
    pub evaluate_name: String,
    /// The number of named child variables.
    /// The client can use this information to present the children in a paged UI
    /// and fetch them in chunks.
    #[serde(rename = "namedVariables")]
    pub named_variables: usize,
    /// The number of indexed child variables.
    /// The client can use this information to present the children in a paged UI
    /// and fetch them in chunks.
    #[serde(rename = "indexedVariables")]
    pub indexed_variables: usize,
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
            id: None,
            name: "undefined".to_string(),
            value: "any".to_string(),
            typing: "Any".to_string(),
            evaluate_name: "".to_string(),
            named_variables: 0,
            indexed_variables: 0,
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
    /// Used to identify variables, the id will be sent later for query
    pub fn with_key(self, id: usize) -> Self {
        Self { id: NonZeroUsize::new(id), ..self }
    }
    /// Create a new variable with a value.
    pub fn with_value<T, V>(self, typing: T, value: V) -> Self
    where
        T: Into<String>,
        V: Into<String>,
    {
        Self { value: value.into(), typing: typing.into(), ..self }
    }
}
