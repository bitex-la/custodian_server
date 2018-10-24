use jsonapi::model::*;
use serde::de::Deserialize;
use serde::ser::Serialize;
use tiny_ram_db::Record;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonApiRecord<T>(pub Record<T>);

impl<T> JsonApiModel for JsonApiRecord<T>
where
    for<'de> T: Deserialize<'de>,
    T: Serialize,
{
    fn jsonapi_type() -> &'static str {
        stringify!(T)
    }
    fn jsonapi_id(&self) -> Option<String> {
        Some(self.0.id.to_string())
    }
    fn relationship_fields() -> Option<&'static [&'static str]> {
        None
    }
    fn build_relationships(&self, _query: &QueryFields) -> Option<Relationships> {
        None
    }
    fn build_included(&self, _fields: &Option<Vec<String>>) -> Option<Resources> {
        None
    }
}
