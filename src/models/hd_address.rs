use std::io::Read;
use std::fmt;

use tiny_ram_db::{ Index, Indexer, Record };
use jsonapi::model::{ JsonApiModel, JsonApiDocument };
use models::jsonapi_record::{ JsonApiRecord, JsonApiResource };
use models::hd_wallet::HdWallet;
use models::resource_address::ResourceAddress;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HdAddress {
    pub public_address: Option<String>,
    pub path: Vec<u64>,
    pub wallet: JsonApiRecord<HdWallet>,
}
from_data!(JsonApiRecord<HdAddress>);

impl ResourceAddress for HdAddress {}

impl fmt::Display for HdAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.public_address.as_ref().map_or("", |id| id))
    }
}

impl JsonApiResource for JsonApiRecord<HdAddress> {
    fn _in_type() -> &'static str { "hd_address" }
}

#[derive(Default)]
pub struct HdAddressIndex {
    by_public_address: Index<Option<String>, HdAddress>,
    by_wallet: Index<Record<HdWallet>, HdAddress>
}

impl Indexer for HdAddressIndex {
    type Item = HdAddress;
    fn index(&mut self, item: &Record<HdAddress>) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_public_address.insert(item.data.public_address.clone(), item.clone())?;
        self.by_wallet.insert(item.data.wallet.0.clone(), item.clone())?;
        Ok(true)
    }
}
