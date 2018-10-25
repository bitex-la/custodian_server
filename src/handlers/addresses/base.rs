use std::str::FromStr;

use bitprim::payment_address::PaymentAddress;
use handlers::handler::{parse_to_value, JsonResult};
use jsonapi::model::*;
use bitprim::executor::Executor;
use models::resource_wallet::ResourceWallet;
use models::transaction::Transaction;
use models::jsonapi_record::JsonApiRecord;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;

pub trait AddressHandler 
where
    Self: serde::Serialize
{
    fn address_index(state: &ServerState, id: u64) -> JsonResult {
        unimplemented!()
    }

    fn address_create<A>(state: &ServerState, id: u64, address: A) -> JsonResult {
        unimplemented!()
    }

    fn address_destroy<A>(state: &ServerState, id: u64, address: A) -> JsonResult {
        unimplemented!()
    }

    fn balance(exec: &Executor, address: String, limit: Option<u64>, since: Option<u64>) -> JsonResult {
        let explorer = exec.explorer();

        if let Ok(valid_address) = PaymentAddress::from_str(&address) {
            match explorer.address_unspents(valid_address, limit.unwrap_or(10_000), since.unwrap_or(0)) {
                Ok(vec_received) => parse_to_value(vec_received.iter().map(|r| r.satoshis).into_iter().sum::<u64>()),
                Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
            }
        } else {
            Err(status::Custom(Status::InternalServerError, "Invalid Address".to_string()))
        }
    }

    fn get_utxos(exec: &Executor, address: String, limit: Option<u64>, since: Option<u64>) -> JsonResult {
        let explorer = exec.explorer();

        if let Ok(valid_address) = PaymentAddress::from_str(&address) {
            match explorer.address_unspents(valid_address, limit.unwrap_or(10_000), since.unwrap_or(0)) {
                Ok(vec_received) => parse_to_value(
                        vec_received.into_iter().map(|received| {
                            Transaction::new(received, address.clone())
                        }).collect::<Vec<Transaction>>()
                ),
                Err(error) => Err(status::Custom(Status::InternalServerError, error.to_string()))
            }
        } else {
            Err(status::Custom(Status::InternalServerError, "Invalid Address".to_string()))
        }
    }
}

impl<R: serde::Serialize> AddressHandler for R {}
