use std::io::Read;
use std::fmt;
use tiny_ram_db::{ Index, Indexer, Record };
use jsonapi::model::*;
use models::jsonapi_record::{ JsonApiRecord };
use models::resource_address::ResourceAddress;
use models::plain_wallet::PlainWallet;
use models::address::Address;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlainAddress {
    pub public_address: Option<String>,
    pub wallet: JsonApiRecord<PlainWallet>,
}

jsonapi_model!(ResourceAddress<PlainAddress>; "address");
from_data!(ResourceAddress<PlainAddress>);

impl Address for PlainAddress { }

impl fmt::Display for PlainAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.public_address.as_ref().map_or("", |id| id))
    }
}

#[derive(Default)]
pub struct AddressIndex {
    by_public_address: Index<Option<String>, PlainAddress>,
    by_wallet: Index<Record<PlainWallet>, PlainAddress>
}

impl Indexer for AddressIndex {
    type Item = PlainAddress;
    fn index(&mut self, item: &Record<PlainAddress>) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_public_address.insert(item.data.public_address.clone(), item.clone())?;
        self.by_wallet.insert(item.data.wallet.0.clone(), item.clone())?;
        Ok(true)
    }
}
