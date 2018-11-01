use std::io::Read;

use bitprim::explorer::Received;
use jsonapi::model::*;
use models::database::Database;
use models::plain_address::PlainAddress;
use models::resource_transaction::JsonApiModelTransaction;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use tiny_ram_db::PlainTable;
use data_guards::FromJsonApiDocument;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromForm)]
pub struct PlainWallet {
    pub version: String,
    pub label: String,
}

impl FromJsonApiDocument for PlainWallet {
    fn from_json_api_document(doc: JsonApiDocument, db: Database) -> Result<Self> {
        let data = doc.data;
        if data.jsonapi_type() != "multisig_wallet" {
            return Err("Type was wrong");
        }

        let version = data.attributes.version;
        let label = data.attributes.label;
        Ok(PlainWallet{version, label})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainUtxo {
    pub prev_hash: String,
    pub prev_index: u32,
    pub address: PlainAddress,
    pub amount: u64,
}

impl JsonApiModelTransaction for PlainUtxo {
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
}
