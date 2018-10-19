use std::fmt;
use std::io::Read;

use bitprim::explorer::Received;
use jsonapi::model::*;
use tiny_ram_db::Table;

use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::database::Database;
use models::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdWallet {
    pub id: Option<u64>,
    pub version: String,
    #[serde(default)]
    pub addresses: Vec<HdAddress>,
    pub xpub: String,
}

jsonapi_model!(HdWallet; "hd_wallet"; has many addresses);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdUtxo {
    pub id: Option<String>,
    pub address: HdAddress,
    pub transaction: Transaction
}
jsonapi_model!(HdUtxo; "hd_utxo");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HdAddress {
    pub id: Option<String>,
    pub address: String,
    pub path: Vec<u64>,
}

impl fmt::Display for HdAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.address)
    }
}

jsonapi_model!(HdAddress; "hd_address");

impl Wallet for HdWallet {
    type Utxo = HdUtxo;
    type RA = HdAddress;

    fn construct_utxo(&self, received: Received, address: &HdAddress) -> Self::Utxo {
        HdUtxo {
            id: Some(format!(
                "{}-{}",
                received.transaction_hash.to_hex(), received.position
            )),
            address: address.clone(),
            transaction: Transaction::new(received, address.to_string())
        }
    }

    fn get_addresses<'a>(&'a self) -> &'a Vec<HdAddress> {
        &self.addresses
    }
}

from_data!(HdWallet);
from_data!(HdAddress);

impl ResourceWallet for HdWallet {
    type A = HdAddress;

    fn raw_id(&self) -> Option<u64> {
        self.id
    }

    fn set_id(self, new_id: u64) -> Self {
        HdWallet {
            id: Some(new_id),
            ..self
        }
    }

    fn merge(self, newer: Self) -> Self {
        let addresses = self.addresses;
        HdWallet {
            addresses,
            id: self.id,
            ..newer
        }
    }

    fn add_address(&mut self, address: Self::A) {
        self.addresses.push(address);
    }

    fn get_addresses<'a>(&'a mut self) -> &'a mut Vec<Self::A> {
        self.addresses.as_mut()
    }

    fn default_fields() -> &'static str {
        "version,xpub"
    }

    fn address_fields() -> &'static str {
        "address, path"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self> {
        &mut database.hd_wallets
    }

    fn remove_address(&mut self, index: usize) {
        self.addresses.remove(index);
    }

    fn find_address_position(&self, address: &Self::A) -> Option<usize> {
        self.addresses
            .clone()
            .into_iter()
            .position(|in_address| in_address.to_string() == address.to_string())
    }
}

impl ResourceAddress for HdAddress {}
