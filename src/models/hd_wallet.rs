use bitprim::explorer::Received;
use jsonapi::model::*;
use tiny_ram_db::{Table, Record, Index, Indexer, hashbrown};
use serde_json;
use tiny_ram_db;

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
    #[serde(skip_deserializing)]
    pub balance: Option<u64>
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

#[derive(Default)]
pub struct HdWalletIndex {
    pub by_label: Index<String, HdWallet>,
}

impl HdWalletIndex {
    fn remove(&mut self, label: String) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_label.data.remove(&label);
        Ok(true)
    }
}

impl Indexer for HdWalletIndex {
    type Item = HdWallet;
    fn index(&mut self, item: &Record<HdWallet>) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_label
            .insert(item.data.label.clone(), item.clone())?;
        Ok(true)
    }
}

impl Wallet for HdWallet {
    type Index = HdWalletIndex;
    type Utxo = HdUtxo;
    type RA = HdAddress;

    fn construct_utxo(&self, received: Received, address: Record<HdAddress>) -> Self::Utxo {
        HdUtxo {
            address: address.clone(),
            transaction: Transaction::new(received, address.data.public_address())
        }
    }

    fn jsonapi_type() -> &'static str {
        "hd_wallet"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index> {
        &mut database.hd_wallets
    }

    fn update_attributes<'a>(&self, version: String, balance: u64) -> Self{
        HdWallet {
            version,
            xpub: self.xpub.clone(),
            label: self.label.clone(),
            balance: Some(balance)
        }
    }

    fn by_label<'a>(label: String, database: &'a mut Database)
        -> Result<hashbrown::HashSet<Record<Self>>, tiny_ram_db::errors::Error> {
            database
                .hd_wallets
                .indexes
                .read()?
                .by_label
                .get(&label, |items| items.clone())
        }

    fn get_label(&self) -> String {
        self.label.clone()
    }

    fn remove_from_indexes<'a>(table: &'a Table<Self, Self::Index>, id: String) -> Result<bool, tiny_ram_db::errors::Error> {
        let mut indexes = table.indexes.write().expect("Error getting write access to indexes");
        indexes.remove(id)?;
        Ok(true)
    }
}

impl ToJsonApi for HdWallet {
    const TYPE : &'static str = "hd_wallets";

    fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
        hashmap!{
            "version".to_string() => serde_json::to_value(&self.version).unwrap(),
            "xpub".to_string() => serde_json::to_value(&self.xpub).unwrap(),
            "label".to_string() => serde_json::to_value(&self.label).unwrap(),
            "balance".to_string() => serde_json::to_value(&self.balance).unwrap()
        }
    }
}

impl FromJsonApi for HdWallet {
    const TYPE : &'static str = "hd_wallets";

    fn from_json_api_resource(resource: Resource, _db: Database) -> Result<Self, String> {
        Ok(HdWallet{
            version: Self::attribute(&resource, "version")?,
            xpub: Self::attribute(&resource, "xpub")?,
            label: Self::attribute(&resource, "label")?,
            balance: None
        })
    }
}
