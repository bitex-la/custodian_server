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

pub trait AddressHandler: ResourceWallet {
    fn address_index(state: &ServerState, id: u64) -> JsonResult {
        let mut database = state.database_lock();
        let haystack = Self::wallets_from_database(&mut database);

        match haystack.find(id as usize) {
            Ok(maybe_wallet) => parse_to_value(vec!["address"]),
            Err(_) => Err(status::Custom(
                Status::NotFound,
                format!("Wallet {:?} Not Found", id),
            )),
        }
    }

    fn address_create(state: &ServerState, id: u64, address: JsonApiRecord<Self::A>) -> JsonResult {
        let mut database = state.database_lock();
        let haystack = Self::wallets_from_database(&mut database);

        match haystack.find(id as usize) {
            Ok(wallet_position) => {
                //TODO: Create Address
                parse_to_value("address")
            }
            Err(_) => Err(status::Custom(
                Status::NotFound,
                format!("Wallet {:?} Not Found", id),
            )),
        }
    }

    fn address_destroy(state: &ServerState, id: u64, address: JsonApiRecord<Self::A>) -> JsonResult {
        let mut database = state.database_lock();
        let haystack = Self::wallets_from_database(&mut database);

        match haystack.find(id as usize) {
            Ok(value) => parse_to_value("address"),
            Err(_) => Err(status::Custom(
                Status::NotFound,
                format!("Wallet with id {:?} Not Found", id),
            )),
        }
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

impl<R: ResourceWallet> AddressHandler for R {}
