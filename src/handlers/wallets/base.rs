use serde;
use jsonapi::model::*;
use bitprim::executor::Executor;
use data_guards::Mapped;
use handlers::helpers::{JsonResult, to_value};
use models::address::Address;
use models::wallet::Wallet;
use models::utxo::Utxo;
use serializers::ToJsonApi;
use rocket::http::Status;
use rocket::response::status;
use serde::de::Deserialize;
use serde::ser::Serialize;
use server_state::ServerState;
use std::sync::Arc;
use tiny_ram_db;
use tiny_ram_db::hashbrown;
use tiny_ram_db::{Record, HashMapRecord, Indexer};

pub trait WalletHandler
where
    Self: serde::Serialize + Wallet,
    for<'de> Self: serde::Deserialize<'de>,
    <Self as Wallet>::Index: Indexer<Item = Self>,
    Self: ToJsonApi,
{
    fn index(state: &ServerState) -> JsonResult {
        let raw_wallets = Self::get_wallets(state)
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;
        
        let wallets = raw_wallets
            .into_iter()
            .map(|(id, record)| {
                let mut wallet = record.data.as_ref().clone();
                if let Ok(addresses) = Self::get_addresses(state, id) {
                    wallet = wallet.update_version(addresses);
                }
                (wallet.get_label(), wallet)
            });
        let hash_set_wallets: JsonApiDocument = Self::collection_to_jsonapi_document(wallets);
        to_value(hash_set_wallets)
    }

    fn get_utxos(
        state: &ServerState,
        id: String,
        limit: Option<u64>,
        since: Option<u64>,
    ) -> JsonResult
    where
        <Self as Wallet>::Utxo: Serialize + Utxo,
        for<'de> <Self as Wallet>::Utxo: Deserialize<'de>,
        <Self as Wallet>::Utxo: ToJsonApi
    {
        WalletHandler::get_transactions(
            state,
            id,
            limit,
            since,
            |executor: &Executor, wallet: &Self, addresses: hashbrown::HashSet<Record<Self::RA>>, limit, since| {
                wallet.get_utxos(executor, addresses, limit, since)
                    .into_iter()
                    .map(|utxo| (utxo.id(), utxo))
                    .collect()
            },
        )
    }

    fn get_incoming(
        state: &ServerState,
        id: String,
        limit: Option<u64>,
        since: Option<u64>,
    ) -> JsonResult {
        WalletHandler::get_transactions(
            state,
            id,
            limit,
            since,
            |executor: &Executor, wallet: &Self, addresses: hashbrown::HashSet<Record<Self::RA>>, limit, since| {
                wallet.get_incoming(executor, addresses, limit, since)
                    .into_iter()
                    .map(|tx| (tx.transaction_hash.clone(), tx))
                    .collect()
            },
        )
    }

    fn get_transactions<F, T>(
        state: &ServerState,
        id: String,
        limit: Option<u64>,
        since: Option<u64>,
        fn_tx: F,
    ) -> JsonResult
    where
        F: FnOnce(&Executor, &Self, hashbrown::HashSet<Record<Self::RA>>, Option<u64>, Option<u64>) -> Vec<(String, T)>,
        T: ToJsonApi,
    {
        match Self::get_wallet_and_addresses(state, id.clone()) {
            Ok((wallet, addresses)) => {
                to_value(Self::collection_to_jsonapi_document(fn_tx(
                    &state.executor,
                    &wallet.data,
                    addresses,
                    limit,
                    since,
                )))
            }
            Err(_) => Err(status::Custom(Status::NotFound, format!("{:?}", id))),
        }
    }

    fn show(state: &ServerState, id: String) -> JsonResult
    {

        let mut record = Self::get_wallet(state, id)
            .map_err(|error| status::Custom(Status::NotFound, error.to_string()))?;

        let addresses = Self::get_addresses(state, record.id)
            .map_err(|error| status::Custom(Status::NotFound, error.to_string()))?;

        record.data = Arc::new(record.data.update_version(addresses));

        to_value(record.data.to_jsonapi_document(record.data.get_label()))
    }

    fn create(state: &ServerState, new: Mapped<Self>) -> JsonResult 
    {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);

        let record = wallets.insert(new.0)
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

        to_value(record.data.to_jsonapi_document(record.id))
    }

    fn update(state: &ServerState, id: String, resource_wallet: Mapped<Self>) -> JsonResult {
        let record = Self::get_wallet(state, id.clone())
            .map_err(|error| status::Custom(Status::NotFound, error.to_string()))?;

        Self::destroy_indexes(state, id)
            .map_err(|error| status::Custom(Status::NotFound, error.to_string()))?;

        Self::update_record(state, record.id, resource_wallet.0)
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

        to_value(true)
    }

    fn destroy_indexes(state: &ServerState, id: String) -> Result<bool, tiny_ram_db::errors::Error> {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);

        Self::remove_from_indexes(wallets, id.clone())
    }

    fn destroy(state: &ServerState, id: String) -> JsonResult {
        let record = Self::get_wallet(state, id.clone())
            .map_err(|error| status::Custom(Status::NotFound, error.to_string()))?;

        Self::destroy_indexes(state, id.clone())
            .map_err(|error| status::Custom(Status::NotFound, error.to_string()))?;

        let record = Self::destroy_record(state, record.id)
            .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

        to_value(record)
    }

    fn destroy_record(state: &ServerState, id: usize) -> Result<Record<Self>, tiny_ram_db::errors::Error> {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);
        let mut records = wallets.data.write().unwrap();

        let record = records.remove(&id)
            .ok_or_else(|| tiny_ram_db::errors::Error::from("RecordNotFound"))?;

        Ok(record)
    }

    fn get_wallet_and_addresses(state: &ServerState, id: String) -> Result<(Record<Self>, hashbrown::HashSet<Record<Self::RA>>), tiny_ram_db::errors::Error> {
        let wallet = Self::get_wallet(state, id)?;
        let addresses = Self::get_addresses(state, wallet.id)?;

        Ok((wallet, addresses))
    }

    fn update_record(state: &ServerState, id: usize, resource_wallet: Self) -> Result<bool, tiny_ram_db::errors::Error> {
        let mut database = state.database_lock();
        let plain_table = Self::wallets_from_database(&mut database);
        let mut records = plain_table.data.write()?;
        records.remove(&id);
        let new_record = Record {
            id: id,
            data: Arc::new(resource_wallet),
        };
        plain_table.indexes.write()?.index(&new_record)?;
        records.insert(id, new_record);
        Ok(true)
    }

    fn get_wallets(state: &ServerState) -> Result<HashMapRecord<Self>, tiny_ram_db::errors::Error> {
        let mut database = state.database_lock();
        let plain_table = Self::wallets_from_database(&mut database);
        let wallets = plain_table.data.read()?;
        Ok(wallets.clone())
    }

    fn get_wallet(state: &ServerState, id: String) -> Result<Record<Self>, tiny_ram_db::errors::Error> {
        let mut database = state.database_lock();
        let wallets = Self::by_label(id, &mut database)?;

        wallets
            .into_iter()
            .nth(0)
            .ok_or_else(|| tiny_ram_db::errors::Error::from("RecordNotFound"))
    }

    fn get_addresses(state: &ServerState, wallet_id: usize) -> Result<hashbrown::HashSet<Record<Self::RA>>, tiny_ram_db::errors::Error> {
        let mut database = state.database_lock();
        let addresses = Self::RA::by_wallet(wallet_id, &mut database)?;

        Ok(addresses)
    }

}

impl<R> WalletHandler for R
where
    R: serde::Serialize + Wallet,
    for<'de> R: serde::Deserialize<'de>,
    <Self as Wallet>::Index: Indexer<Item = Self>,
    R: ToJsonApi,
{
}
