use serde::de::Deserialize;
use serde::ser::Serialize;
use jsonapi::model::{ JsonApiModel, Relationships, QueryFields, Resources };
use models::wallet::Wallet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceWallet<W: Wallet> {
    pub id: Option<usize>,
    pub wallet: W
}

impl<W> JsonApiModel for ResourceWallet<W>
where
    for<'de> W: Deserialize<'de>,
    W: Serialize,
    W: Wallet
{
    fn jsonapi_type() -> &'static str {
        W::jsonapi_type()
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

impl<W: Wallet> ResourceWallet<W> {
    fn empty() -> ResourceWallet<W> {
        ResourceWallet {
            id: None,
            wallet: W::empty()
        }
    }
}