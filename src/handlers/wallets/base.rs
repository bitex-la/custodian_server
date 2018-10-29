use serde::ser::Serialize;
use serde::de::Deserialize;
use bitprim::executor::Executor;
use handlers::handler::{
    check_resource_operation, from_record_to_resource_wallet, parse_to_value,
    plain_table_to_jsonapi, JsonResult,
};
use jsonapi::model::{vec_to_jsonapi_document, JsonApiModel};
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;
use std::sync::Arc;
use tiny_ram_db::Record;

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
    for<'de> <Self as Wallet>::Utxo: Deserialize<'de>
    {
        WalletHandler::get_transactions(
            state,
            id,
            limit,
            since,
            |executor: &Executor, wallet: &&Self, limit, since| {
                wallet.get_utxos(executor, limit, since)
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
            |executor: &Executor, wallet: &&Self, limit, since| {
                wallet.get_incoming(executor, limit, since)
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
        F: FnOnce(&Executor, &&Self, Option<u64>, Option<u64>) -> Vec<T>,
        T: JsonApiModel,
    {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);

        match wallets.find(id) {
            Ok(wallet) => parse_to_value(vec_to_jsonapi_document(fn_tx(
                &state.executor,
                &&*wallet.data,
                limit,
                since,
            ))),
            Err(_) => Err(status::Custom(Status::NotFound, format!("{:?}", id))),
        }
    }

    fn show(state: &ServerState, id: usize) -> JsonResult
    where
        ResourceWallet<Self>: JsonApiModel,
    {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);

        from_record_to_resource_wallet(wallets.find(id))
    }

    fn create(state: &ServerState, new: ResourceWallet<Self>) -> JsonResult {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);

        check_resource_operation(wallets.insert(new.wallet))
    }

    fn update(state: &ServerState, id: usize, resource_wallet: ResourceWallet<Self>) -> JsonResult {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);

        let mut vec_records = wallets.data.write().unwrap();
        vec_records.remove(id);
        let new_record = Record {
            id: id,
            data: Arc::new(resource_wallet.wallet),
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
}

impl<R> WalletHandler for R
where
    R: serde::Serialize + Wallet,
    for<'de> R: serde::Deserialize<'de>,
{
}
