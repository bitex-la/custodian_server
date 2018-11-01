use std::io::Read;
use std::fmt;
use std::collections::HashSet;

use tiny_ram_db::{ Index, Indexer, Record, Table };
use jsonapi::model::*;
use models::hd_wallet::HdWallet;
use models::address::Address;
use models::resource_address::ResourceAddress;
use models::database::Database;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HdAddress {
    pub public_address: Option<String>,
    pub path: Vec<u64>,
    pub wallet: Record<HdWallet>,
}

jsonapi_model!(ResourceAddress<HdAddress, HdWallet>; "hd_address"; has one wallet);

impl FromJsonApiDocument for HdAddress {
    fn from_json_api_document(doc: JsonApiDocument, db: Database) -> Result<Self> {
        let data = doc.data;
        if data.type != "hd_addresses" {
            bail!("Type was wrong");
        }

        let public_address = data.attributes.public_address;
        let path = data.attributes.path;
        let wallet = db.hd_wallets.find(data.relationships.wallet.data.id);
        Ok(HdAddress{public_address, path, wallet})
    }
}

impl Address for HdAddress {
    type Index = HdAddressIndex;
    type Wallet = HdWallet;

    fn addresses_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index> {
        &mut database.hd_addresses
    }

    fn jsonapi_type() -> &'static str {
        "hd_address"
    }

    fn filter_by_wallet<'a>(
        wallet_id: usize,
        database: &'a mut Database,
    ) -> Result<HashSet<Record<Self>>, tiny_ram_db::errors::Error> {
        let wallet = database.hd_wallets.find(wallet_id)?;
        database
            .hd_addresses
            .indexes
            .read()?
            .by_wallet
            .get(&wallet, |items| items.clone())
    }

    fn get_record_wallet(&self) -> Record<Self::Wallet> {
        self.wallet.clone()
    }
}

impl fmt::Display for HdAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.public_address.as_ref().map_or("", |id| id))
    }
}

#[derive(Default)]
pub struct HdAddressIndex {
    by_public_address: Index<Option<String>, HdAddress>,
    by_wallet: Index<Record<HdWallet>, HdAddress>
}

impl Indexer for HdAddressIndex {
    type Item = HdAddress;
    fn index(&mut self, item: &Record<HdAddress>) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_public_address.insert(item.data.public_address.clone(), item.clone())?;
        self.by_wallet.insert(item.data.wallet.clone(), item.clone())?;
        Ok(true)
    }
}
