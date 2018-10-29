use std::io::Read;

use bitprim::explorer::Received;
use jsonapi::model::*;
use models::database::Database;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::jsonapi_record::{ JsonApiRecord, JsonApiResource };
use tiny_ram_db::{ PlainTable };
use models::plain_address::PlainAddress;
use models::resource_transaction::JsonApiModelTransaction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlainWallet {
    pub version: String,
    pub label: String,
}

from_data!(ResourceWallet<PlainWallet>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainUtxo {
    pub prev_hash: String,
    pub prev_index: u32,
    pub address: PlainAddress,
    pub amount: u64,
}

impl JsonApiModelTransaction  for PlainUtxo {
    fn jsonapi_type() -> &'static str {
        "plain_utxo"
    }
}

impl Wallet for PlainWallet {
    type Utxo = PlainUtxo;
    type RA = PlainAddress;

    fn construct_utxo(&self, received: Received, address: &PlainAddress) -> Self::Utxo {
        PlainUtxo {
            prev_hash: received.transaction_hash.to_hex(),
            prev_index: received.position,
            address: address.clone(),
            amount: received.satoshis,
        }
    }

    fn jsonapi_type() -> &'static str {
        "plain_wallet"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self> {
        &mut database.plain_wallets
    }

    fn empty() -> Self {
        PlainWallet {
            version: "".to_string(),
            label: "".to_string()
        }
    }
}
