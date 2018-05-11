use std::io::Read;
use rocket::{Request, Data};
use rocket::data::{self, FromData};
use rocket::Outcome::*;
use rocket::http::Status;
use serde_json;

use jsonapi::model::*;
use models::plain_wallet::PlainWallet;
use models::hd_wallet::HdWallet;
use models::multisig_wallet::MultisigWallet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallets {
    pub id: String,
    pub plain: Vec<PlainWallet>,
    pub hd: Vec<HdWallet>,
    pub multisig: Vec<MultisigWallet>
}

jsonapi_model!(Wallets; "wallets"; has many plain, hd, multisig);

impl FromData for Wallets {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Self, String> {

        let mut string_wallets = String::new();
        if let Err(e) = data.open().read_to_string(&mut string_wallets) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        let raw_json: JsonApiDocument = match serde_json::from_str(&string_wallets) {
            Ok(value)  => value,
            Err(err) => return Failure((Status::BadRequest, format!("{:?}", err)))
        };

        match Self::from_jsonapi_document(&raw_json) {
            Ok(wallets) => Success(wallets),
            Err(err) => return Failure((Status::BadRequest, format!("{:?}", err)))
        }
    }
}
