use std::io::Read;
use std::fmt;

use tiny_ram_db::{ Index, Indexer, Record };
use jsonapi::model::*;
use models::jsonapi_record::*;
use models::address::Address;
use models::multisig_wallet::MultisigWallet;
use models::resource_address::ResourceAddress;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisigAddress {
    pub public_address: Option<String>,
    pub path: Vec<u64>,
    pub wallet: JsonApiRecord<MultisigWallet>,
}
jsonapi_model!(ResourceAddress<MultisigAddress>; "multisig_address");
from_data!(ResourceAddress<MultisigAddress>);

impl Address for MultisigAddress { }

impl fmt::Display for MultisigAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.public_address.as_ref().map_or("", |id| id))
    }
}

#[derive(Default)]
pub struct MultisigAddressIndex {
    by_public_address: Index<Option<String>, MultisigAddress>,
    by_wallet: Index<Record<MultisigWallet>, MultisigAddress>
}

impl Indexer for MultisigAddressIndex {
    type Item = MultisigAddress;
    fn index(&mut self, item: &Record<MultisigAddress>) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_public_address.insert(item.data.public_address.clone(), item.clone())?;
        self.by_wallet.insert(item.data.wallet.0.clone(), item.clone())?;
        Ok(true)
    }
}
