use bitprim::executor::Executor;
use jsonapi::model::*;
use models::wallet::Wallet;
use models::resource_wallet::ResourceWallet;

use std::io::Read;
use rocket::data::{self, FromData};
use rocket::{Request, Data};
use rocket::http::Status;
use rocket::Outcome::*;
use serde_json;

pub type Address = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainWallet {
    pub id: String,
    pub version: String,
    pub addresses: Vec<Address>
}

jsonapi_model!(PlainWallet; "plain_wallet");

#[derive(Debug)]
pub struct PlainUtxo {
    pub prev_hash: String,
    pub prev_index: u64,
    pub address: String,
    pub amount: u64
}

impl Wallet for PlainWallet {
    type Utxo = PlainUtxo;

    fn get_utxos(&self, _exec: &Executor) -> Vec<Self::Utxo> {
        vec![
            PlainUtxo {
                prev_hash: "abc".to_string(),
                prev_index: 1,
                address: "abc".to_string(),
                amount: 100000000,
            },
        ]
    }
}

from_data_wallet!(PlainWallet);

impl ResourceWallet for PlainWallet {
    fn id(&self) -> i32 {
        self.id.parse::<i32>().unwrap_or(0)
    }
}
