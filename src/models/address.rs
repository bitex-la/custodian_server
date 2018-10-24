use std::io::Read;
use std::fmt;
use tiny_ram_db::{ Index, Indexer, Record };
use jsonapi::model::{ JsonApiModel, JsonApiDocument };
use models::jsonapi_record::{ JsonApiRecord, JsonApiResource };
use models::resource_address::ResourceAddress;
use models::plain_wallet::PlainWallet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub public_address: Option<String>,
    pub wallet: JsonApiRecord<PlainWallet>,
}
from_data!(JsonApiRecord<Address>);

impl ResourceAddress for Address {}
impl fmt::Display for Address {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.public_address.as_ref().map_or("", |id| id))
    }
}

impl JsonApiResource for JsonApiRecord<Address> {
    fn _in_type() -> &'static str { "address" }
}

#[derive(Default)]
pub struct AddressIndex {
    by_public_address: Index<Option<String>, Address>,
    by_wallet: Index<Record<PlainWallet>, Address>
}

impl Indexer for AddressIndex {
    type Item = Address;
    fn index(&mut self, item: &Record<Address>) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_public_address.insert(item.data.public_address.clone(), item.clone())?;
        self.by_wallet.insert(item.data.wallet.0.clone(), item.clone())?;
        Ok(true)
    }
}
