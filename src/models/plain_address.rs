use jsonapi::model::*;
use models::address::Address;
use models::database::Database;
use models::plain_wallet::PlainWallet;
use models::resource_address::ResourceAddress;
use std::collections::HashSet;
use std::fmt;
use std::io::Read;
use tiny_ram_db;
use tiny_ram_db::{Index, Indexer, Record, Table};
use data_guards::FromJsonApiDocument;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlainAddress {
    pub public_address: Option<String>,
    pub wallet: Record<PlainWallet>,
}

jsonapi_model!(ResourceAddress<PlainAddress, PlainWallet>; "address"; has one wallet);

impl FromJsonApiDocument for PlainAddress {
    fn from_json_api_document(doc: JsonApiDocument, db: Database) -> Result<Self, String> {
        let data = doc.data;
        if data.jsonapi_type() != "plain_address" {
            return Err("Type was wrong");
        }

        let public_address = data.attributes.public_address;
        let wallet = db.hd_wallets.find(data.relationships.wallet.data.id);
        Ok(PlainAddress{public_address, wallet})
    }
}
impl Address for PlainAddress {
    type Index = AddressIndex;
    type Wallet = PlainWallet;

    fn addresses_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index> {
        &mut database.plain_addresses
    }

    fn jsonapi_type() -> &'static str {
        "plain_address"
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

impl fmt::Display for PlainAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.public_address.as_ref().map_or("", |id| id))
    }
}

#[derive(Default)]
pub struct AddressIndex {
    by_public_address: Index<Option<String>, PlainAddress>,
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
