use std::fmt;
use std::io::Read;

use bitprim::explorer::Received;
use jsonapi::model::*;
use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::wallets::Wallets;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub id: Option<String>,
}
impl ResourceAddress for Address {}
impl fmt::Display for Address {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.id.as_ref().map_or("", |id| id))
    }
}
jsonapi_model!(Address; "address");
from_data!(Address);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainWallet {
    pub id: Option<u64>,
    pub version: String,
    #[serde(default)]
    pub addresses: Vec<Address>,
}

jsonapi_model!(PlainWallet; "plain_wallet"; has many addresses);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainUtxo {
    pub id: Option<String>,
    pub prev_hash: String,
    pub prev_index: u32,
    pub address: Address,
    pub amount: u64,
}
jsonapi_model!(PlainUtxo; "plain_utxo"; has one address);

impl Wallet for PlainWallet {
    type Utxo = PlainUtxo;
    type RA = Address;

    fn construct_utxo(&self, received: Received, address: &Address) -> Self::Utxo {
        PlainUtxo {
            id: Some(format!(
                "{}-{}",
                received.transaction_hash, received.position
            )),
            prev_hash: received.transaction_hash,
            prev_index: received.position,
            address: address.clone(),
            amount: received.satoshis,
        }
    }

    fn get_addresses<'a>(&'a self) -> &'a Vec<Address> {
        &self.addresses
    }
}

from_data!(PlainWallet);

impl ResourceWallet for PlainWallet {
    type A = Address;

    fn raw_id(&self) -> Option<u64> {
        self.id
    }

    fn set_id(self, new_id: u64) -> Self {
        PlainWallet {
            id: Some(new_id),
            ..self
        }
    }

    fn merge(self, newer: Self) -> Self {
        let addresses = self.addresses;
        PlainWallet {
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
        "version"
    }

    fn address_fields() -> &'static str {
        "id"
    }

    fn collection_from_wallets<'a>(wallets: &'a mut Wallets) -> &'a mut Vec<Self> {
        wallets.plains.as_mut()
    }

    fn remove_address(&mut self, index: usize) {
        self.addresses.swap_remove(index);
    }

    fn find_address_position(&self, address: &Self::A) -> Option<usize> {
        self.addresses
            .clone()
            .into_iter()
            .position(|in_address| in_address.id == address.id)
    }
}
