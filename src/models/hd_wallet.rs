use bitprim::explorer::Received;
use jsonapi::model::*;
use tiny_ram_db::{PlainTable, Record, hashbrown};
use serde_json;

use models::wallet::Wallet;
use models::hd_address::HdAddress;
use models::database::Database;
use models::transaction::Transaction;
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
    pub address: Record<HdAddress>,
    pub transaction: Transaction
}

impl ToJsonApi for HdUtxo {
    const TYPE : &'static str = "hd_utxos";

    fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
        hashmap!{
            "transaction".to_string() => serde_json::to_value(&self.transaction).unwrap()
        }
    }

    fn relationships(&self, _fields: &QueryFields) -> Option<Relationships> {
        Some(hashmap!{
            "address".to_string() => Self::has_one("hd_addresses", self.address.id),
        })
    }

    fn included(&self, _fields: &Vec<String>) -> Option<Resources> {
        Some(vec![self.address.data.to_jsonapi_resource(self.address.id).0])
    }
}

impl Wallet for HdWallet {
    type Utxo = HdUtxo;
    type RA = HdAddress;

    fn construct_utxo(&self, received: Received, address: Record<HdAddress>) -> Self::Utxo {
        HdUtxo {
            address: address.clone(),
            transaction: Transaction::new(received, address.data.public())
        }
    }

    fn jsonapi_type() -> &'static str {
        "hd_wallet"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self> {
        &mut database.hd_wallets
    }

    fn update_version<'a>(&self, addresses: hashbrown::HashSet<Record<Self::RA>>) -> Self{
        let version = addresses.len().to_string();
        HdWallet {
            version,
            xpub: self.xpub.clone(),
            label: self.label.clone()
        }
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
