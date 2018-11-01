use std::io::Read;

use bitprim::explorer::Received;
use jsonapi::model::*;
use tiny_ram_db::PlainTable;

use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::hd_address::HdAddress;
use models::database::Database;
use models::transaction::Transaction;
use models::resource_transaction::JsonApiModelTransaction;
use data_guards::FromJsonApiDocument;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HdWallet {
    pub version: String,
    pub xpub: String,
    pub label: String,
}

impl FromJsonApiDocument for HdWallet {
    fn from_json_api_document(doc: JsonApiDocument, db: Database) -> Result<Self> {
        let data = doc.data;
        if data.jsonapi_type() != "hd_wallet" {
            return Err("Type was wrong");
        }

        let version = data.attributes.version;
        let xpub = data.attributes.xpub;
        let label = data.attributes.label;
        Ok(HdWallet{version, xpub, label})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdUtxo {
    pub address: HdAddress,
    pub transaction: Transaction
}

impl JsonApiModelTransaction  for HdUtxo {
    fn jsonapi_type() -> &'static str {
        "hd_utxo"
    }
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

    fn jsonapi_type() -> &'static str {
        "hd_wallet"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self> {
        &mut database.hd_wallets
    }
}
