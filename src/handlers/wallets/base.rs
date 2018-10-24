use bitprim::executor::Executor;
use handlers::handler::{parse_to_value, JsonResult};
use jsonapi::model::*;
use models::hd_wallet::HdWallet;
use models::resource_wallet::ResourceWallet;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;
use models::jsonapi_record::JsonApiRecord;

pub trait WalletHandler: ResourceWallet {
    //FIXME: Fix return value
    fn index(state: &ServerState) -> JsonResult {
        unimplemented!()
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

    fn create(state: &ServerState, new: JsonApiRecord<Self>) -> JsonResult {
        unimplemented!()
    }

    fn update(state: &ServerState, id: u64, new: JsonApiRecord<Self>) -> JsonResult {
        unimplemented!()
    }

    fn destroy(state: &ServerState, id: u64) -> JsonResult {
        unimplemented!()
    }
}

impl<R: ResourceWallet> WalletHandler for R {}
