use jsonapi::model::*;
use models::address::Address;
use models::database::Database;
use models::plain_wallet::PlainWallet;
use std::collections::HashSet;
use tiny_ram_db;
use tiny_ram_db::{Index, Indexer, Record, Table};
use data_guards::FromJsonApiDocument;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlainAddress {
    pub public_address: String,
    pub wallet: Record<PlainWallet>,
}

impl Address for PlainAddress {
    type Index = AddressIndex;
    type Wallet = PlainWallet;

    fn public(&self) -> String {
        self.public_address.clone()
    }

    fn addresses_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index> {
        &mut database.plain_addresses
    }

    fn filter_by_wallet<'a>(
        wallet_id: usize,
        database: &'a mut Database,
    ) -> Result<HashSet<Record<Self>>, tiny_ram_db::errors::Error> {
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
}

#[derive(Default)]
pub struct AddressIndex {
    by_public_address: Index<String, PlainAddress>,
    by_wallet: Index<Record<PlainWallet>, PlainAddress>,
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
    const TYPE : &'static str = "plain_addresses";

		fn relationships(&self, _fields: &QueryFields) -> Option<Relationships> {
				Some(hashmap!{
						"wallet" => Self::has_one("wallets", self.wallet.id),
				})
    }

		fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
				hashmap!{
						"public_address" => serde_json::to_value(self.public_address).unwrap()
				}
		}

		fn included(&self, _fields: &QueryFields) -> Option<Resources> {
				Some(vec![self.wallet.data.to_jsonapi_resource(self.wallet.id)])
		}
}

impl FromJsonApi for PlainAddress {
    const TYPE : &'static str = "plain_addresses";

    fn from_json_api_resource(resource: Resource, db: Database) -> Result<Self, String> {
        let public_address = Self::attribute(&resource, "public_address")?;
        let wallet_id = Self::has_one_id(&resource, "wallet")?;
        let wallet = db.plain_wallets.find(wallet_id)
            .map_err(|_| format!("Wallet not found"))?;
        Ok(PlainAddress{public_address, wallet})
    }
}
