use std::str::FromStr;

use bitprim::executor::Executor;
use bitprim::payment_address::PaymentAddress;
use handlers::handler::{
    check_resource_operation, from_record_to_resource_address, parse_to_value, table_to_jsonapi,
    JsonResult,
};
use jsonapi::model::*;
use models::address::Address;
use models::resource_address::ResourceAddress;
use models::transaction::Transaction;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;
use tiny_ram_db;

#[derive(FromForm, Debug)]
pub struct AddressFilters {
    pub wallet_id: Option<usize>,
}

pub trait AddressHandler
where
    Self: serde::Serialize + Address,
    ResourceAddress<Self, <Self as Address>::Wallet>: JsonApiModel,
    <Self as Address>::Index: tiny_ram_db::Indexer<Item = Self>,
{
    fn index(state: &ServerState, filters: AddressFilters) -> JsonResult {
        let mut database = state.database_lock();
        match filters.wallet_id {
            Some(wallet_id) => {
                if let Ok(addresses) = Self::filter_by_wallet(wallet_id, &mut database) {
                    parse_to_value(addresses)
                } else {
                    Err(status::Custom(
                        Status::NotFound,
                        "Wallet not found".to_string(),
                    ))
                }
            }
            None => {
                let addresses = Self::addresses_from_database(&mut database);
                table_to_jsonapi(addresses)
            }
        }
    }

    fn create(
        state: &ServerState,
        new: Self,
    ) -> JsonResult {
        let mut database = state.database_lock();
        let addresses = Self::addresses_from_database(&mut database);

        check_resource_operation(addresses.insert(new))
    }

    fn show(state: &ServerState, id: usize) -> JsonResult
    where
        ResourceAddress<Self, <Self as Address>::Wallet>: JsonApiModel,
    {
        let mut database = state.database_lock();
        let addresses = Self::addresses_from_database(&mut database);

        from_record_to_resource_address(addresses.find(id))
    }

    //TODO: Naive version
    fn destroy(state: &ServerState, id: usize) -> JsonResult {
        let mut database = state.database_lock();
        let addresses = Self::addresses_from_database(&mut database);

        let mut vec_records = addresses.data.write().unwrap();
        vec_records.remove(id);

        parse_to_value(true)
    }

    fn balance(
        exec: &Executor,
        address: String,
        limit: Option<u64>,
        since: Option<u64>,
    ) -> JsonResult {
        let explorer = exec.explorer();

        if let Ok(valid_address) = PaymentAddress::from_str(&address) {
            match explorer.address_unspents(
                valid_address,
                limit.unwrap_or(10_000),
                since.unwrap_or(0),
            ) {
                Ok(vec_received) => {
                    parse_to_value(vec_received.iter().map(|r| r.satoshis).sum::<u64>())
                }
                Err(error) => Err(status::Custom(
                    Status::InternalServerError,
                    error.to_string(),
                )),
            }
        } else {
            Err(status::Custom(
                Status::InternalServerError,
                "Invalid Address".to_string(),
            ))
        }
    }

    fn get_utxos(
        exec: &Executor,
        address: String,
        limit: Option<u64>,
        since: Option<u64>,
    ) -> JsonResult {
        let explorer = exec.explorer();

        if let Ok(valid_address) = PaymentAddress::from_str(&address) {
            match explorer.address_unspents(
                valid_address,
                limit.unwrap_or(10_000),
                since.unwrap_or(0),
            ) {
                Ok(vec_received) => parse_to_value(
                    vec_received
                        .into_iter()
                        .map(|received| Transaction::new(received, address.clone()))
                        .collect::<Vec<Transaction>>(),
                ),
                Err(error) => Err(status::Custom(
                    Status::InternalServerError,
                    error.to_string(),
                )),
            }
        } else {
            Err(status::Custom(
                Status::InternalServerError,
                "Invalid Address".to_string(),
            ))
        }
    }
}

impl<R> AddressHandler for R
where
    R: serde::Serialize + Address,
    ResourceAddress<R, <R as Address>::Wallet>: JsonApiModel,
    <R as Address>::Index: tiny_ram_db::Indexer<Item = Self>,
{
}
