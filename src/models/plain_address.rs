use jsonapi::model::*;
use models::address::Address;
use models::database::Database;
use models::plain_wallet::PlainWallet;
use serializers::{FromJsonApi, ToJsonApi};
use serde_json;
use tiny_ram_db;
use tiny_ram_db::hashbrown;
use tiny_ram_db::{Index, Indexer, Record, Table};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlainAddress {
    pub public_address: String,
    #[serde(skip_deserializing)]
    pub balance: Option<u64>,
    pub wallet: Record<PlainWallet>,
}

impl Address for PlainAddress {
    type Index = AddressIndex;
    type Wallet = PlainWallet;

    fn public(&self) -> String {
        self.public_address.clone()
    }

    fn table<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index> {
        &mut database.plain_addresses
    }

    fn by_wallet<'a>(
        wallet_id: usize,
        database: &'a mut Database,
        ) -> Result<hashbrown::HashSet<Record<Self>>, tiny_ram_db::errors::Error> {
        let wallet = database.plain_wallets.find(wallet_id)?;
        database
            .plain_addresses
            .indexes
            .read()?
            .by_wallet
            .get(&wallet, |items| items.clone())
    }

    fn get_record_wallet(&self) -> Record<Self::Wallet> {
        self.wallet.clone()
    }

    fn remove_from_indexes<'a>(table: &'a Table<Self, Self::Index>, id: &'a usize) -> Result<bool, tiny_ram_db::errors::Error> {
        let mut indexes = table.indexes.write().expect("Error getting write access to indexes");
        indexes.remove(table, id)?;
        Ok(true)
    }

    fn update_attributes<'a>(&self, balance: u64) -> Self {
        PlainAddress { 
            balance: Some(balance), 
            public_address: self.public_address.clone(),
            wallet: self.wallet.clone()
        }
    }
}

#[derive(Default)]
pub struct AddressIndex {
    by_public_address: Index<String, PlainAddress>,
    by_wallet: Index<Record<PlainWallet>, PlainAddress>,
}

impl AddressIndex {
    fn remove(&mut self, table: &Table<PlainAddress, AddressIndex>, id: &usize) -> Result<bool, tiny_ram_db::errors::Error> {
        let address = table.find(id.clone())?;

        self.by_public_address.data.remove(&address.data.public_address);
        if let Some(wallet) = self.by_wallet.data.get_mut(&address.data.wallet) {
            wallet.remove(&address);
        }

        Ok(true)
    }
}

impl Indexer for AddressIndex {
    type Item = PlainAddress;
    fn index(&mut self, item: &Record<PlainAddress>) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_public_address
            .insert(item.data.public_address.clone(), item.clone())?;
        self.by_wallet
            .insert(item.data.wallet.clone(), item.clone())?;
        Ok(true)
    }
}

impl ToJsonApi for PlainAddress {
    const TYPE: &'static str = "plain_addresses";

    fn relationships(&self, _fields: &QueryFields) -> Option<Relationships> {
        Some(hashmap!{
            "wallet".to_string() => Self::has_one("plain_wallets", self.wallet.data.label.clone()),
        })
    }

    fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
        hashmap!{
            "public_address".to_string() => serde_json::to_value(&self.public_address).unwrap(),
            "balance".to_string() => serde_json::to_value(&self.balance).unwrap()
        }
    }

    fn included(&self, _fields: &Vec<String>) -> Option<Resources> {
        Some(vec![self.wallet.data.to_jsonapi_resource(self.wallet.data.label.clone()).0])
    }
}

impl FromJsonApi for PlainAddress {
    const TYPE: &'static str = "plain_addresses";

    fn from_json_api_resource(resource: Resource, db: Database) -> Result<Self, String> {
        let public_address = Self::attribute(&resource, "public_address")?;
        let wallet_id = Self::has_one_id(&resource, "wallet")?;
        let data = db
                .plain_wallets
                .indexes
                .read()
                .map_err(|_| format!("Wallet not found"))?;
        let wallet = data
                .by_label
                .get(&wallet_id, |items| items.clone())
            .map_err(|_| format!("Wallet not found"))?
            .into_iter()
            .nth(0)
            .ok_or_else(|| format!("Wallet not found"))?;
        Ok(PlainAddress {
            public_address,
            wallet,
            balance: None
        })
    }
}
