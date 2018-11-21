use serde;
use std::str::FromStr;

use bitprim::executor::Executor;
use bitprim::payment_address::PaymentAddress;
use handlers::helpers::{JsonResult, to_value};
use jsonapi::model::*;
use models::address::Address;
use models::transaction::Transaction;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;
use tiny_ram_db::Indexer;
use serializers::ToJsonApi;

/* This trait is the base of all the address handlers, it should only
 * take care of receiving the request input like filters and fields,
 * and serializing the output to jsonapi.
 */
pub trait AddressHandler
where
    Self: serde::Serialize + Address,
    <Self as Address>::Index: Indexer<Item = Self>,
    Self: ToJsonApi
{

    fn index(state: &ServerState) -> JsonResult 
    {
        let mut database = state.database_lock();
        let table = Self::table(&mut database);
        let result = table.data.read()
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

        let addresses = result
            .iter()
            .map(|(id, record)| (id, record.data.as_ref().clone()));
        let hash_set_addresses: JsonApiDocument = Self::collection_to_jsonapi_document(addresses);
        to_value(hash_set_addresses)
    }

    fn create(state: &ServerState, new: Self) -> JsonResult {
        let mut database = state.database_lock();

        let record = Self::table(&mut database)
            .insert(new.clone())
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

        to_value(record)
    }

    fn show(state: &ServerState, id: usize) -> JsonResult {
        let mut database = state.database_lock();
        let addresses = Self::table(&mut database);

        let record = addresses.find(id)
            .map_err(|error| status::Custom(Status::NotFound, error.to_string()))?;

        to_value(record.data.to_jsonapi_document(record.id))
    }

    fn destroy(state: &ServerState, id: usize) -> JsonResult {
        let mut database = state.database_lock();
        let addresses = Self::table(&mut database);

        Self::remove_from_indexes(addresses, &id)
            .map_err(|error| status::Custom(Status::NotFound, error.to_string()))?;

        let mut records = addresses.data.write().expect("Error getting write access to addresses");

        let record = records.remove(&id)
            .ok_or_else(|| status::Custom(Status::InternalServerError, "Could not remove".to_string()))?;

        to_value(record)
    }

    fn balance(
        exec: &Executor,
        address: String,
        limit: Option<u64>,
        since: Option<u64>,
        ) -> JsonResult {
        let explorer = exec.explorer();

        let valid_address = PaymentAddress::from_str(&address)
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

        let balance = explorer.address_unspents(valid_address, limit.unwrap_or(10_000), since.unwrap_or(0))
            .map(|vec_received| vec_received.iter().map(|r| r.satoshis).sum::<u64>())
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

        to_value(hashmap!{
            "data" => hashmap!{
                "type" => "balance".to_string(), 
                "amount" => balance.to_string()
            }
        })
    }

    fn get_utxos(
        exec: &Executor,
        address: String,
        limit: Option<u64>,
        since: Option<u64>,
        ) -> JsonResult {
        let explorer = exec.explorer();

        let valid_address = PaymentAddress::from_str(&address)
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

        let transactions = explorer.address_unspents(valid_address, limit.unwrap_or(10_000), since.unwrap_or(0))
            .map(|vec_received| vec_received.into_iter().map(|received| Transaction::new(received, address.clone())).collect::<Vec<Transaction>>())
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

        to_value(transactions)
    }
}

impl<R> AddressHandler for R
where
    R: serde::Serialize + Address,
    <R as Address>::Index: Indexer<Item = Self>,
    Self: ToJsonApi
{
}
