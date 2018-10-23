use std::fmt;
use std::io::Read;

use bitprim::explorer::Received;
use jsonapi::model::*;
use models::database::Database;
use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::jsonapi_record::JsonApiRecord;
use tiny_ram_db::{PlainTable, Record};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub public_address: Option<String>,
    pub wallet: Record<PlainWallet>,
}

impl ResourceAddress for Address {}
impl fmt::Display for Address {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.public_address.as_ref().map_or("", |id| id))
    }
}
from_data!(Address);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainWallet {
    pub version: String,
    pub label: String,
}

impl JsonApiModel for JsonApiRecord<PlainWallet>
{
    fn jsonapi_type() -> &'static str {
        "plain_wallets"
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

from_data!(PlainWallet);

impl ResourceWallet for PlainWallet {
    type A = Address;

    fn default_fields() -> &'static str {
        "version"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self> {
        &mut database.plain_wallets
    }
}
