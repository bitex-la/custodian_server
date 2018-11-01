use std::io::Read;
use std::fmt;
use std::collections::HashSet;

use tiny_ram_db::{ Index, Indexer, Record, Table };
use jsonapi::model::*;
use models::hd_wallet::HdWallet;
use models::address::Address;
use models::resource_address::ResourceAddress;
use models::database::Database;
use data_guards::FromJsonApiDocument;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HdAddress {
    pub public_address: String,
    pub path: Vec<u64>,
    pub wallet: Record<HdWallet>,
}

jsonapi_model!(ResourceAddress<HdAddress, HdWallet>; "hd_address"; has one wallet);

impl FromJsonApiDocument for HdAddress {
    fn from_json_api_document(doc: JsonApiDocument, db: Database) -> Result<Self, String> {
        if let Some(PrimaryData::Single(resource)) = doc.data {
            if resource._type != "hd_addresses" {
                return Err("Type was wrong".into());
            }

            let public_address = if let Some(serde_json::Value::String(value)) = resource.attributes.get("public_address") {
                value.clone()
            } else  {
                return Err("No public address".into())
            };
            let path: Vec<u64> = if let Some(value) = resource.attributes.get("path") {
                match serde_json::from_value(*value) {
                    Ok(path) => path,
                    Err(error) => return Err("Error parsing Path".into())
                }
            } else {
                return Err("No path".into())
            };
            let relationships = if let Some(relationship) = resource.relationships {
                relationship
            } else {
                return Err("No wallet".into())
            };
            let wallet = if let Some(Relationship{ data: IdentifierData::Single(identifier), .. }) = relationships.get("wallet") {
                match identifier.id.parse::<usize>() {
                    Ok(value) => match db.hd_wallets.find(value) {
                        Ok(wallet) => wallet,
                        Err(error) => return Err("Wallet Not Found".into())
                    },
                    Err(error) => return Err("Invalid Wallet Id".into())
                }
            } else {
                return Err("Failed getting wallet id".into())
            };

            Ok(HdAddress{public_address, path, wallet})
        } else {
            Err("Invalid document data".into())
        }
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
