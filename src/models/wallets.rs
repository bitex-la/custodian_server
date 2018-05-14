use std::mem;
use std::io::Read;
use std::iter::Iterator;
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

macro_rules! find_wallet_and_perform_action {
    ( $this:ty, $wallet_type:ty, $wallet_item:expr, $wallets:expr, $action:expr ) => {
        {
            let mut not_found_wallet: Option<$wallet_type> = None;

            for wallet in $wallets {
                match $wallet_item.iter().position(|w| w.id == wallet.id ) {
                    Some(index) => Some($action(index, wallet)),
                    None        => { not_found_wallet = Some(wallet.clone()); None }
                };
            }

            if let Some(wallet) = not_found_wallet {
                Err(format!("{:?}", wallet))
            } else {
                Ok(true)
            }
        }
    };
}

macro_rules! update_wallet {
    ( $this:ty, $wallet_type:ty, $wallet_item:expr, $wallets:expr ) => {
        {
            find_wallet_and_perform_action!($this, $wallet_type, $wallet_item, $wallets, |index, wallet| {
                mem::replace(&mut $wallet_item[index], wallet)
            })
        }
    };
}

macro_rules! delete_wallet {
    ( $this:ty, $wallet_type:ty, $wallet_item:expr, $wallets:expr ) => {
        {

            find_wallet_and_perform_action!($this, $wallet_type, $wallet_item, $wallets, |index, _| {
                $wallet_item.remove(index);
                true
            })
        }
    };
}

impl Wallets {
    pub fn create(&mut self, wallets: Wallets) {
        self.plain.extend(wallets.plain);
        self.hd.extend(wallets.hd);
        self.multisig.extend(wallets.multisig);
    }

    pub fn update(&mut self, wallets: Wallets) -> Result<bool, String> {
        update_wallet!(self, PlainWallet, &mut self.plain, wallets.plain)?;
        update_wallet!(self, HdWallet, &mut self.hd, wallets.hd)?;
        update_wallet!(self, MultisigWallet, &mut self.multisig, wallets.multisig)
    }

    pub fn destroy(&mut self, wallets: Wallets) -> Result<bool, String> {
        delete_wallet!(self, PlainWallet, &mut self.plain, wallets.plain)?;
        delete_wallet!(self, HdWallet, &mut self.hd, wallets.hd)?;
        delete_wallet!(self, MultisigWallet, &mut self.multisig, wallets.multisig)
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
