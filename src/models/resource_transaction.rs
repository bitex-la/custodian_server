use serde::de::Deserialize;
use serde::ser::Serialize;
use jsonapi::model::{ JsonApiModel, Relationships, QueryFields, Resources };

pub trait JsonApiModelTransaction {
    fn jsonapi_type() -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceTransaction<T: JsonApiModelTransaction> {
    pub id: Option<usize>,
    pub transaction: T
}

impl<T> JsonApiModel for ResourceTransaction<T>
where
    for<'de> T: Deserialize<'de>,
    T: Serialize + JsonApiModelTransaction
{
    fn jsonapi_type() -> &'static str {
        T::jsonapi_type()
    }
    fn jsonapi_id(&self) -> Option<String> {
        self.id.map(|_id| _id.to_string())
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