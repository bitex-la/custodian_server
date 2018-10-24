use std::fmt;
use std::io::Read;

use bitprim::explorer::Received;
use jsonapi::model::*;
use tiny_ram_db::PlainTable;

use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::database::Database;
use models::transaction::Transaction;
use models::jsonapi_record::{ JsonApiRecord, JsonApiResource };

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HdWallet {
    pub version: String,
    pub xpub: String,
    pub label: String,
}

from_data!(JsonApiRecord<HdWallet>);

impl JsonApiResource for JsonApiRecord<HdWallet> {
    fn _in_type() -> &'static str { "hd_wallet" }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdUtxo {
    pub address: HdAddress,
    pub transaction: Transaction
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HdAddress {
    pub public_address: Option<String>,
    pub path: Vec<u64>,
    pub wallet: JsonApiRecord<HdWallet>,
}
from_data!(JsonApiRecord<HdAddress>);

impl fmt::Display for HdAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.public_address.as_ref().map_or("", |id| id))
    }
}

impl JsonApiResource for JsonApiRecord<HdAddress> {
    fn _in_type() -> &'static str { "hd_address" }
}

impl Wallet for HdWallet {
    type Utxo = HdUtxo;
    type RA = HdAddress;

    fn construct_utxo(&self, received: Received, address: &HdAddress) -> Self::Utxo {
        HdUtxo {
            address: address.clone(),
            transaction: Transaction::new(received, address.to_string())
        }
    }
}

impl ResourceWallet for HdWallet {
    type A = HdAddress;

    fn default_fields() -> &'static str {
        "version,xpub"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self> {
        &mut database.hd_wallets
    }
}

impl ResourceAddress for HdAddress {}
