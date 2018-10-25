use std::io::Read;

use bitprim::explorer::Received;
use jsonapi::model::*;
use tiny_ram_db::PlainTable;

use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::hd_address::HdAddress;
use models::database::Database;
use models::transaction::Transaction;
use models::jsonapi_record::{ JsonApiRecord, JsonApiResource };

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HdWallet {
    pub version: String,
    pub xpub: String,
    pub label: String,
}

from_data!(ResourceWallet<HdWallet>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdUtxo {
    pub address: HdAddress,
    pub transaction: Transaction
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

    fn _in_type() -> &'static str { "hd_wallet" }

    fn default_fields() -> &'static str {
        "version,xpub"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self> {
        &mut database.hd_wallets
    }
}
