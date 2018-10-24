use std::fmt;
use std::io::Read;

use bitprim::explorer::Received;
use jsonapi::model::*;
use models::database::Database;
use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::jsonapi_record::{ JsonApiRecord, JsonApiResource };
use tiny_ram_db::{PlainTable, Record};

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlainWallet {
    pub version: String,
    pub label: String,
}

from_data!(JsonApiRecord<PlainWallet>);

impl JsonApiResource for JsonApiRecord<PlainWallet> {
    fn _in_type() -> &'static str { "plain_wallet" }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainUtxo {
    pub prev_hash: String,
    pub prev_index: u32,
    pub address: Address,
    pub amount: u64,
}

impl Wallet for PlainWallet {
    type Utxo = PlainUtxo;
    type RA = Address;

    fn construct_utxo(&self, received: Received, address: &Address) -> Self::Utxo {
        PlainUtxo {
            prev_hash: received.transaction_hash.to_hex(),
            prev_index: received.position,
            address: address.clone(),
            amount: received.satoshis,
        }
    }
}

impl ResourceWallet for PlainWallet {
    type A = Address;

    fn default_fields() -> &'static str {
        "version"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self> {
        &mut database.plain_wallets
    }
}
