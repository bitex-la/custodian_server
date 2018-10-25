use bitprim::executor::Executor;
use handlers::handler::{ parse_to_value, JsonResult, check_resource_operation };
use models::wallet::Wallet;
use models::hd_wallet::HdWallet;
use models::jsonapi_record::JsonApiRecord;
use models::resource_wallet::ResourceWallet;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;

pub trait WalletHandler
where
    Self: serde::Serialize + Wallet,
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

    fn show(state: &ServerState, id: u64) -> JsonResult {
        unimplemented!()
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

impl<R: serde::Serialize + Wallet> WalletHandler for R {}
