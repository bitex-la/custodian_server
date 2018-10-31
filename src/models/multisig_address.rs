use std::io::Read;
use std::fmt;
use std::collections::HashSet;

use tiny_ram_db::{ Index, Indexer, Record, Table };
use jsonapi::model::*;
use models::address::Address;
use models::multisig_wallet::MultisigWallet;
use models::resource_address::ResourceAddress;
use models::database::Database;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisigAddress {
    pub public_address: Option<String>,
    pub path: Vec<u64>,
    pub wallet: Record<MultisigWallet>,
}
jsonapi_model!(ResourceAddress<MultisigAddress, MultisigWallet>; "multisig_address");
from_data!(ResourceAddress<MultisigAddress, MultisigWallet>);

impl Address for MultisigAddress {
    type Index = MultisigAddressIndex;
    type Wallet = MultisigWallet;

    fn addresses_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index> {
        &mut database.multisig_addresses
    }

    fn jsonapi_type() -> &'static str {
        "multisig_address"
    }

    fn filter_by_wallet<'a>(
        wallet_id: usize,
        database: &'a mut Database,
    ) -> Result<HashSet<Record<Self>>, tiny_ram_db::errors::Error> {
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

impl fmt::Display for MultisigAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.public_address.as_ref().map_or("", |id| id))
    }
}

#[derive(Default)]
pub struct MultisigAddressIndex {
    by_public_address: Index<Option<String>, MultisigAddress>,
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
