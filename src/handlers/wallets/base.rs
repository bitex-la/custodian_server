use jsonapi::model::JsonApiModel;
use bitprim::executor::Executor;
use handlers::handler::{ parse_to_value, JsonResult, check_resource_operation, from_record_to_resource_wallet };
use models::wallet::Wallet;
use models::resource_wallet::ResourceWallet;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;

pub trait WalletHandler
where
    Self: serde::Serialize + Wallet,
    for<'de> Self: serde::Deserialize<'de>
{
    fn index(state: &ServerState) -> JsonResult {
        let mut database = state.database_lock();
        let wallets = Self::wallets_from_database(&mut database);
        parse_to_value(wallets)
    }

    fn get_utxos(
        state: &ServerState,
        id: u64,
        limit: Option<u64>,
        since: Option<u64>,
    ) -> JsonResult {
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
        id: u64,
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
        id: u64,
        limit: Option<u64>,
        since: Option<u64>,
        fn_tx: F,
    ) -> JsonResult
    where
        F: FnOnce(&Executor, &&Self, Option<u64>, Option<u64>) -> Vec<T>,
    {
        unimplemented!()
    }

    fn show(state: &ServerState, id: usize) -> JsonResult 
    where ResourceWallet<Self>: JsonApiModel
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

    fn update(
        state: &ServerState,
        id: u64,
        new: ResourceWallet<Self>,
    ) -> JsonResult {
        unimplemented!()
    }

    fn destroy(state: &ServerState, id: u64) -> JsonResult {
        unimplemented!()
    }
}

impl<R> WalletHandler for R 
where
    R: serde::Serialize + Wallet,
    for<'de> R: serde::Deserialize<'de>
{}
