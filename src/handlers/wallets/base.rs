use bitprim::executor::Executor;
use std::collections::HashSet;
use data_guards::Mapped;
use handlers::handler::JsonResult;
use models::address::Address;
use models::wallet::Wallet;
use rocket::http::Status;
use rocket::response::status;
use serde::de::Deserialize;
use serde::ser::Serialize;
use server_state::ServerState;
use std::sync::Arc;
use tiny_ram_db::{Record};
use serializers::*;

pub trait WalletHandler
where
    Self: serde::Serialize + Wallet,
    for<'de> Self: serde::Deserialize<'de>,
{
    fn index(state: &ServerState) -> JsonResult {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);
        plain_table_to_jsonapi(wallets)
    }

    fn get_utxos(
        state: &ServerState,
        id: usize,
        limit: Option<u64>,
        since: Option<u64>,
    ) -> JsonResult
    where
        <Self as Wallet>::Utxo: Serialize,
        for<'de> <Self as Wallet>::Utxo: Deserialize<'de>,
    {
        WalletHandler::get_transactions(
            state,
            id,
            limit,
            since,
            |executor: &Executor, wallet: &&Self, addresses: HashSet<Record<Self::RA>>, limit, since| {
                wallet.get_utxos(executor, addresses, limit, since)
            },
        )
    }

    fn get_incoming(
        state: &ServerState,
        id: usize,
        limit: Option<u64>,
        since: Option<u64>,
    ) -> JsonResult {
        WalletHandler::get_transactions(
            state,
            id,
            limit,
            since,
            |executor: &Executor, wallet: &&Self, addresses: HashSet<Record<Self::RA>>, limit, since| {
                wallet.get_incoming(executor, addresses, limit, since)
            },
        )
    }

    fn get_transactions<F, T>(
        state: &ServerState,
        id: usize,
        limit: Option<u64>,
        since: Option<u64>,
        fn_tx: F,
    ) -> JsonResult
    where
        F: FnOnce(&Executor, &&Self, HashSet<Record<Self::RA>>, Option<u64>, Option<u64>) -> Vec<T>,
        T: JsonApiModel,
    {
        match Self::get_wallet_and_addresses(state, id) {
            Ok((wallet, addresses)) => {
                parse_to_value(vec_to_jsonapi_document(fn_tx(
                    &state.executor,
                    &&*wallet.data,
                    addresses,
                    limit,
                    since,
                )))
            }
            Err(_) => Err(status::Custom(Status::NotFound, format!("{:?}", id))),
        }
    }

    fn show(state: &ServerState, id: usize) -> JsonResult {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);

        from_record_to_resource_wallet(wallets.find(id))
    }

    fn create(state: &ServerState, new: Mapped<Self>) -> JsonResult {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);

        check_resource_operation(wallets.insert(new.0))
    }

    fn update(state: &ServerState, id: usize, resource_wallet: Mapped<Self>) -> JsonResult {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);

        let mut vec_records = wallets.data.write().unwrap();
        vec_records.remove(id);
        let new_record = Record {
            id,
            data: Arc::new(resource_wallet.0),
        };
        vec_records.insert(id, new_record);

        parse_to_value(true)
    }

    //TODO: Naive version
    fn destroy(state: &ServerState, id: usize) -> JsonResult {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);

        let mut vec_records = wallets.data.write().unwrap();
        vec_records.remove(id);

        parse_to_value(true)
    }

    fn get_wallet_and_addresses(state: &ServerState, id: usize) -> Result<(Record<Self>, HashSet<Record<Self::RA>>), tiny_ram_db::errors::Error> {
        let wallet = Self::get_wallet(state, id)?;
        let addresses = Self::get_addresses(state, wallet.id)?;

        Ok((wallet, addresses))
    }

    fn get_wallet(state: &ServerState, id: usize) -> Result<Record<Self>, tiny_ram_db::errors::Error> {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);
        let wallet = wallets.find(id)?;

        Ok(wallet)
    }

    fn get_addresses(state: &ServerState, wallet_id: usize) -> Result<HashSet<Record<Self::RA>>, tiny_ram_db::errors::Error> {
        let mut database = state.database_lock();
        let addresses = Self::RA::filter_by_wallet(wallet_id, &mut database)?;

        Ok(addresses)
    }

}

impl<R> WalletHandler for R
where
    R: serde::Serialize + Wallet,
    for<'de> R: serde::Deserialize<'de>,
{
}
