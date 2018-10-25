use jsonapi::model::{ JsonApiModel, QueryFields, Relationships, Resources };
use models::database::Database;
use models::resource_address::ResourceAddress;
use models::wallet::Wallet;
use serde::de::Deserialize;
use serde::ser::Serialize;
use tiny_ram_db::PlainTable;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceWallet<W> (pub W);

impl<W> JsonApiModel for ResourceWallet<W>
where
    for<'de> W: Deserialize<'de>,
    W: Serialize,
    Self: Wallet
{
    fn jsonapi_type() -> &'static str {
        Self::_in_type()
    }
    fn jsonapi_id(&self) -> Option<String> {
        None
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