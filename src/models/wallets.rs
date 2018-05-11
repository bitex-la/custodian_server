use std::mem;
use std::io::Read;
use rocket::{Request, Data};
use rocket_contrib::{Json, Value};
use rocket::data::{self, FromData};
use rocket::Outcome::*;
use rocket::response::status;
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

macro_rules! update_wallet {
    ( $this:ty, $wallet_type:ty, $wallet_item:expr, $wallets:expr ) => {
        {
            let mut not_found_wallet: Option<$wallet_type> = None;

            for wallet in $wallets {
                match $wallet_item.iter().position(|w| w.id == wallet.id ) {
                    Some(index) => Some(mem::replace(&mut $wallet_item[index], wallet)),
                    None        => { not_found_wallet = Some(wallet.clone()); None }
                };
            }

            if let Some(wallet) = not_found_wallet {
                Err(status::NotFound(format!("{:?}", wallet)))
            } else {
                Ok(Json(json!({"status": "ok"})))
            }
        }
    };
}

impl Wallets {
    pub fn update_plain_wallets(&mut self, plain_wallets: Vec<PlainWallet>) -> Result<Json<Value>, status::NotFound<String>> {
        update_wallet!(self, PlainWallet, &mut self.plain, plain_wallets)
    }

    pub fn update_hd_wallets(&mut self, hd_wallets: Vec<HdWallet>) -> Result<Json<Value>, status::NotFound<String>> {
        update_wallet!(self, HdWallet, &mut self.hd, hd_wallets)
    }

    pub fn update_multisig_wallets(&mut self, multisig_wallets: Vec<MultisigWallet>) -> Result<Json<Value>, status::NotFound<String>> {
        update_wallet!(self, MultisigWallet, &mut self.multisig, multisig_wallets)
    }

}

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
