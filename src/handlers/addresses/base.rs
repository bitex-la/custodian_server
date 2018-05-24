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
    where
    {
        let mut wallets = state.wallets_lock();
        let haystack = Self::collection_from_wallets(&mut wallets);

        match haystack.iter().position(|wallet| wallet.id() == id) {
            Some(value) => { 
                match haystack[value].get_addresses().into_iter().find(|in_address| in_address == &address) {
                    Some(found_address) => 
                        Err(status::Custom(
                                Status::InternalServerError,
                                format!("Address {:?} is duplicated", found_address),
                                )),
                    None    => { 
                        haystack[value].add_address(address);
                        match haystack[value].get_addresses().last() {
                            Some(last_address) => Ok(Json(to_value(last_address.to_jsonapi_document()).unwrap_or(Value::Null))),
                            None => Err(status::Custom(Status::NotFound, format!("Address Not Found")))
                        }
                        
                    }
                }
            },
            None => Err(status::Custom(Status::NotFound, format!("Wallet {:?} Not Found", id)))
        }
    }
}

impl<R: ResourceWallet> AddressHandler for R {}
