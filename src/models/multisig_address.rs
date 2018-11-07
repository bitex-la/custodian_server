use std::collections::HashSet;
use serde_json;
use tiny_ram_db;

use tiny_ram_db::{ Index, Indexer, Record, Table };
use jsonapi::model::*;
use models::address::Address;
use models::multisig_wallet::MultisigWallet;
use models::database::Database;
use serializers::{FromJsonApi, ToJsonApi};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisigAddress {
    pub public_address: String,
    pub path: Vec<u64>,
    pub wallet: Record<MultisigWallet>,
}

impl Address for MultisigAddress {
    type Index = MultisigAddressIndex;
    type Wallet = MultisigWallet;

    fn public(&self) -> String {
        self.public_address.clone()
    }

    fn table<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index>
    {
        &mut database.multisig_addresses
    }

    fn by_wallet<'a>( wallet_id: usize, database: &'a mut Database)
        -> Result<HashSet<Record<Self>>, tiny_ram_db::errors::Error>
        {
            let wallet = database.multisig_wallets.find(wallet_id)?;
            database
                .multisig_addresses
                .indexes
                .read()?
                .by_wallet
                .get(&wallet, |items| items.clone())
        }

    fn get_record_wallet(&self) -> Record<Self::Wallet> {
        self.wallet.clone()
    }
}

#[derive(Default)]
pub struct MultisigAddressIndex {
    by_public_address: Index<String, MultisigAddress>,
    by_wallet: Index<Record<MultisigWallet>, MultisigAddress>
}

impl Indexer for MultisigAddressIndex {
    type Item = MultisigAddress;
    fn index(&mut self, item: &Record<MultisigAddress>) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_public_address.insert(item.data.public_address.clone(), item.clone())?;
        self.by_wallet.insert(item.data.wallet.clone(), item.clone())?;
        Ok(true)
    }
}

impl ToJsonApi for MultisigAddress {
    const TYPE : &'static str = "multisig_addresses";

    fn relationships(&self, _fields: &QueryFields) -> Option<Relationships> {
        Some(hashmap!{
            "wallet".to_string() => Self::has_one("multisig_wallets", self.wallet.id),
        })
    }

    fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
        hashmap!{
            "public_address".to_string() => serde_json::to_value(&self.public_address).unwrap(),
            "path".to_string() => serde_json::to_value(&self.path).unwrap()
        }
    }

    fn included(&self, _fields: &Vec<String>) -> Option<Resources> {
        Some(vec![self.wallet.data.to_jsonapi_resource(self.wallet.id).0])
    }
}

impl FromJsonApi for MultisigAddress {
    const TYPE : &'static str = "multisig_addresses";

    fn from_json_api_resource(resource: Resource, db: Database) -> Result<Self, String> {
        let public_address = Self::attribute(&resource, "public_address")?;
        let path = Self::attribute(&resource, "path")?;
        let wallet_id = Self::has_one_id(&resource, "wallet")?;
        let wallet = db.multisig_wallets.find(wallet_id)
            .map_err(|_| format!("Wallet not found"))?;
        Ok(MultisigAddress{public_address, path, wallet})
    }
}

