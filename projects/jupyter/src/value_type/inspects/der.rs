use super::*;

impl<'de> Deserialize<'de> for InspectVariableRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(InspectVariableRequestVisitor::default())
    }
}

#[derive(Default)]
struct InspectVariableRequestVisitor {
    id: Option<NonZeroUsize>,
    filter: Option<InspectVariableFilter>,
    start: usize,
    limit: Option<NonZeroUsize>,
}

impl<'de> Visitor<'de> for InspectVariableRequestVisitor {
    type Value = InspectVariableRequest;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("InspectVariableRequest")
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "filter" => self.filter = Some(map.next_value()?),
            }
        }

        Ok(InspectVariableRequest { id: self.id.unwrap(), filter: self.filter, start: self.start, limit: self.limit })
    }
}
