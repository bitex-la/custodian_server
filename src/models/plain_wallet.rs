use bitprim::executor::Executor;
use jsonapi::model::*;
use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;

use rocket::data::{self, FromData};
use rocket::http::Status;
use rocket::Outcome::*;
use rocket::{Data, Request};
use serde_json;
use std::io::Read;

pub type Address = String;
impl ResourceAddress for Address {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainWallet {
    pub id: String,
    pub version: String,
    pub addresses: Vec<Address>,
}

jsonapi_model!(PlainWallet; "plain_wallet");

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
        vec![PlainUtxo {
            prev_hash: "abc".to_string(),
            prev_index: 1,
            address: "abc".to_string(),
            amount: 100000000,
        }]
    }
}

from_data_wallet!(PlainWallet);

impl ResourceWallet<Address> for PlainWallet {
    fn id(&self) -> i32 {
        self.id.parse::<i32>().unwrap_or(0)
    }

    fn add_address(&mut self, address: Address) {
        self.addresses.push(address);
    }

    fn get_addresses(&self) -> Vec<Address> {
        self.addresses.clone()
    }

    fn remove_address(&mut self, index: usize) {
        self.addresses.remove(index);
    }
}
