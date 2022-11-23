use super::*;
use crate::dap::VariableFilter;
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserializer,
    __private::de::Content,
};
use std::{fmt::Formatter, num::NonZeroUsize};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariablesArguments {
    /// The variable for which to retrieve its children. The `variablesReference`
    /// must have been obtained in the current suspended state. See 'Lifetime of
    /// Object References' in the Overview section for details.
    pub variables_reference: NonZeroUsize,
    /// Filter to limit the child variables to either named or indexed. If omitted,
    /// both types are fetched.
    /// Values: 'indexed', 'named'
    pub filter: Option<VariableFilter>,
    /// The index of the first variable to return; if omitted children start at 0.
    /// The attribute is only honored by a debug adapter if the corresponding
    /// capability `supportsVariablePaging` is true.
    pub start: usize,
    /// The number of variables to return. If count is missing or 0, all variables
    /// are returned.
    /// The attribute is only honored by a debug adapter if the corresponding
    /// capability `supportsVariablePaging` is true.
    pub count: Option<NonZeroUsize>,
    /// Specifies details on how to format the Variable values.
    /// The attribute is only honored by a debug adapter if the corresponding
    /// capability `supportsValueFormattingOptions` is true.
    pub format: ValueFormat,
}

impl<'de> Deserialize<'de> for VariablesArguments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(VariablesArgumentsVisitor::default())
    }
}

#[derive(Default)]
struct VariablesArgumentsVisitor {
    variables_reference: Option<NonZeroUsize>,
    filter: Option<VariableFilter>,
    start: usize,
    count: Option<NonZeroUsize>,
}

impl<'de> Visitor<'de> for VariablesArgumentsVisitor {
    type Value = VariablesArguments;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("InspectVariableRequest")
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "variablesReference" => self.variables_reference = Some(map.next_value()?),
                "filter" => self.filter = Some(map.next_value()?),
                "start" => self.start = map.next_value()?,
                "count" => {
                    let value = map.next_value::<usize>()?;
                    self.count = NonZeroUsize::new(value)
                }
                _ => {
                    let value = map.next_value::<Content>()?;
                    println!("Unknown key {:?}, value {:#?}", key, value)
                }
            }
        }
        if self.variables_reference.is_none() {
            return Err(Error::missing_field("variablesReference"));
        }
        Ok(VariablesArguments {
            variables_reference: self.variables_reference.unwrap(),
            filter: self.filter,
            start: self.start,
            count: self.count,
            format: Default::default(),
        })
    }
}
