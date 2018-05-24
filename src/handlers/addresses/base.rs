use serde_json::to_value;

use jsonapi::model::*;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use server_state::ServerState;
use models::resource_wallet::ResourceWallet;

pub type JsonResult = Result<Json<Value>, status::Custom<String>>;

pub trait AddressHandler : ResourceWallet {

    fn create(
        state: &ServerState,
        id: u64,
        address: Self::A
        ) -> JsonResult
        {
            let mut wallets = state.wallets_lock();
            let haystack = Self::collection_from_wallets(&mut wallets);

            match haystack.iter().position(|wallet| wallet.id() == id) {
                Some(wallet_position) => { 
                    match haystack[wallet_position].find_address_position(&address) {
                        Some(_) => Err(status::Custom(Status::InternalServerError, format!("Duplicate address {:?}", address))),
                        None => {
                            haystack[wallet_position].add_address(address);
                            match haystack[wallet_position].get_addresses().last() {
                                Some(last_address) => Ok(Json(to_value(last_address.to_jsonapi_document()).unwrap_or(Value::Null))),
                                None => Err(status::Custom(Status::InternalServerError, "Problem adding address".to_string()))
                            }
                            
                        } 
                    }
                },
                None => Err(status::Custom(Status::NotFound, format!("Wallet {:?} Not Found", id)))
            }
        }


    fn destroy(
        state: &ServerState,
        id: u64,
        address: Self::A,
        ) -> JsonResult 
    {
        let mut wallets = state.wallets_lock();
        let haystack = Self::collection_from_wallets(&mut wallets);

        match haystack.iter().position(|wallet| wallet.id() == id) {
            Some(value) => {
                match haystack[value].find_address_position(&address) {
                    Some(position) => {
                        haystack[value].remove_address(position);
                        Ok(Json(to_value(address.to_jsonapi_document()).unwrap_or(Value::Null)))
                    },
                    None => Err(status::Custom(Status::NotFound, format!("Address not found {:?}", address)))
                }

            },
            None => Err(status::Custom(Status::NotFound, format!("Wallet with id {:?} Not Found", id)))
        }
    }
}

impl<R: ResourceWallet> AddressHandler for R {}
