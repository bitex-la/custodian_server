use serde;
use std::str::FromStr;
use std::sync::{Arc, MutexGuard};

use tiny_ram_db::hashbrown;
use bitprim::executor::Executor;
use bitprim::payment_address::PaymentAddress;
use handlers::helpers::{JsonResult, to_value};
use jsonapi::model::*;
use models::address::Address;
use models::wallet::Wallet;
use models::transaction::Transaction;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;
use tiny_ram_db::{Indexer, Record};
use serializers::ToJsonApi;
use models::database::Database;

#[derive(FromForm, Debug)]
pub struct AddressFilters {
    pub wallet_id: Option<usize>,
}

pub enum VersionMutation {
    Increment,
    Decrement
}

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

    fn index(state: &ServerState, filters: AddressFilters) -> JsonResult 
    {
        let mut db = state.database_lock();
        let addresses = if let Some(wallet_id) = filters.wallet_id {
            Self::by_wallet(wallet_id, &mut db)
                .map_err(|_| status::Custom(Status::NotFound, format!("Wallet Not Found")))?
        } else {
            hashbrown::HashSet::new()
        };
        let hash_set_addresses: JsonApiDocument = Self::collection_to_jsonapi_document(addresses);
        to_value(hash_set_addresses)
    }

    fn create(state: &ServerState, new: Self) -> JsonResult {
        let mut database = state.database_lock();

        let record = Self::table(&mut database)
            .insert(new.clone())
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

        Self::mutate_version(VersionMutation::Increment, &new, &mut database, &record);

        to_value(record)
    }

    fn show(state: &ServerState, id: usize) -> JsonResult {
        let mut database = state.database_lock();
        let addresses = Self::table(&mut database);

        let address = addresses.find(id)
            .map_err(|error| status::Custom(Status::NotFound, error.to_string()))?;

        to_value(address.to_jsonapi_document(address.id))
    }

    fn destroy(state: &ServerState, id: usize) -> JsonResult {
        let mut database = state.database_lock();
        let addresses = Self::table(&mut database);

        let mut records = addresses.data.write().unwrap();
        records.remove(&id);

        to_value(true)
    }

    fn mutate_version(version: VersionMutation, _self: &Self, database: &mut MutexGuard<Database>, record_address: &Record<Self>) {
        let id = &_self.get_record_wallet().id;
        let wallets = Self::Wallet::wallets_from_database(database);

        let mut records = wallets.data.write().unwrap();

        records.remove(id);
        let mut record_wallet = record_address.data.get_record_wallet().data;
        let wallet = Arc::make_mut(&mut record_wallet);
        match version {
            VersionMutation::Increment => wallet.incr_version(),
            VersionMutation::Decrement => wallet.decr_version()
        }
        let new_record = Record {
            id: id.clone(),
            data: Arc::new(wallet.clone())
        };
        records.insert(id.clone(), new_record);
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

        to_value(balance)
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
