use serde_lsp::dap::VariableFilter;
use std::{fmt::Debug, num::NonZeroUsize};

/// A request to inspect a variable.
#[derive(Copy, Clone, Debug)]
pub struct InspectVariableRequest {
    /// The variable for which to retrieve its children. The `variablesReference`
    /// must have been obtained in the current suspended state. See 'Lifetime of
    /// Object References' in the Overview section for details.
    pub id: NonZeroUsize,
    /// Filter to limit the child variables to either named or indexed. If omitted,
    /// both types are fetched.
    /// Values: 'indexed', 'named'
    pub filter: Option<VariableFilter>,
    /// The index of the first variable to return; if omitted children start at 0.
    /// The attribute is only honored by a debug adapter if the corresponding
    /// capability `supportsVariablePaging` is true.
    pub start: usize,
    /// The number of variables to return. If count is None, all variables are returned.
    pub limit: Option<NonZeroUsize>,
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
#[derive(Clone, Debug)]
pub struct InspectVariable {
    /// The identifier of the variable. If it is empty, then the reference to this variable will not be sent later.
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
    pub typing: String,
    /// The number of named child variables.
    /// The client can use this information to present the children in a paged UI
    /// and fetch them in chunks.
    pub named_variables: usize,
    /// The number of indexed child variables.
    /// The client can use this information to present the children in a paged UI
    /// and fetch them in chunks.
    pub indexed_variables: usize,
    /// A memory reference to a location appropriate for this result.
    /// For pointer type eval results, this is generally a reference to the
    /// memory address contained in the pointer.
    pub memory_reference: usize,
}

impl Default for InspectVariable {
    fn default() -> Self {
        Self {
            id: None,
            name: "undefined".to_string(),
            value: "any".to_string(),
            typing: "Any".to_string(),
            named_variables: 0,
            indexed_variables: 0,
            memory_reference: 0,
        }
    }
}

/// Add module
#[derive(Clone, Debug)]
pub struct InspectModule {
    /// The module's identifier.
    pub id: u32,
    /// The module's name.
    pub name: String,
    /// The module's path.
    pub path: String,
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
    pub fn with_value<V>(self, value: V) -> Self
    where
        V: Into<String>,
    {
        Self { value: value.into(), ..self }
    }
    /// Create a new variable with a value.
    pub fn with_type<T>(self, typing: T) -> Self
    where
        T: Into<String>,
    {
        Self { typing: typing.into(), ..self }
    }
}
