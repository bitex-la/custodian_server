use std::io::Read;

use bitprim::executor::Executor;
use jsonapi::model::*;
use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::wallets::Wallets;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
  pub id: Option<String>
}
impl ResourceAddress for Address {}
jsonapi_model!(Address; "address");
from_data_wallet!(Address);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainWallet {
    pub id: Option<u64>,
    pub version: String,
    #[serde(default)]
    pub addresses: Vec<Address>,
}

jsonapi_model!(PlainWallet; "plain_wallet"; has many addresses);


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dog {
    pub id: Option<u64>,
    pub name: String
}
jsonapi_model!(Dog; "dogs");

#[derive(Debug)]
pub struct PlainUtxo {
    pub prev_hash: String,
    pub prev_index: u64,
    pub address: Address,
    pub amount: u64,
}

impl Wallet for PlainWallet {
    type Utxo = PlainUtxo;

    fn get_utxos(&self, _exec: &Executor) -> Vec<Self::Utxo> {
        vec![]
        /*
        vec![PlainUtxo {
            prev_hash: "abc".to_string(),
            prev_index: 1,
            address: Address{id: "abc".to_string()},
            amount: 100000000,
        }]
        */
    }
}

from_data_wallet!(PlainWallet);

impl ResourceWallet for PlainWallet {
    type A = Address;

    fn raw_id(&self) -> Option<u64> {
        self.id
    }

    fn set_id(self, new_id: u64) -> Self {
        PlainWallet { id: Some(new_id), ..self }
    }

    fn merge(self, newer: Self) -> Self {
      let addresses = self.addresses;
      PlainWallet{ addresses, ..newer }
    }

    fn add_address(&mut self, address: Self::A) {
        self.addresses.push(address);
    }

    fn get_addresses(&self) -> Vec<Self::A> {
        self.addresses.clone()
    }

    fn default_fields() -> &'static str {
      "version"
    }

    fn collection_from_wallets<'a>(wallets: &'a mut Wallets) -> &'a mut Vec<Self> {
      wallets.plains.as_mut()
    }

    fn remove_address(&mut self, address: Self::A) -> Result<bool, String> {
        match self
            .addresses
            .clone()
            .into_iter()
            .position(|in_address| in_address == address)
        {
            Some(index) => {
                self.addresses.remove(index);
                Ok(true)
            }
            None => Err(format!("Address {:?} does not exists", address)),
        }
    }
}
