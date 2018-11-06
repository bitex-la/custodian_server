use bitprim::explorer::Received;
use jsonapi::model::*;
use tiny_ram_db::PlainTable;

use models::wallet::Wallet;
use models::hd_address::HdAddress;
use models::database::Database;
use models::transaction::Transaction;
use models::resource_transaction::JsonApiModelTransaction;
use models::address::Address;
use serializers::{FromJsonApi, ToJsonApi};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HdWallet {
    pub version: String,
    pub xpub: String,
    pub label: String,
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
            transaction: Transaction::new(received, address.public())
        }
    }

    fn jsonapi_type() -> &'static str {
        "hd_wallet"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self> {
        &mut database.hd_wallets
    }
}

impl ToJsonApi for HdWallet {
    const TYPE : &'static str = "hd_wallets";

    fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
        hashmap!{
            "version".to_string() => serde_json::to_value(&self.version).unwrap(),
            "xpub".to_string() => serde_json::to_value(&self.xpub).unwrap(),
            "label".to_string() => serde_json::to_value(&self.label).unwrap()
        }
    }
}

impl FromJsonApi for HdWallet {
    const TYPE : &'static str = "hd_wallets";

    fn from_json_api_resource(resource: Resource, _db: Database) -> Result<Self, String> {
        Ok(HdWallet{
            version: Self::attribute(&resource, "version")?,
            xpub: Self::attribute(&resource, "xpub")?,
            label: Self::attribute(&resource, "label")?
        })
    }
}
