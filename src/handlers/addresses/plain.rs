use handlers::addresses::base::AddressHandler;
use handlers::handler::GetTransactionParams;
use handlers::handler::JsonResult;
use models::address::Address;
use models::plain_wallet::PlainWallet;
use models::jsonapi_record::JsonApiRecord;
use server_state::ServerState;

#[get("/plain_wallets/<id>/relationships/addresses")]
pub fn index(state: &ServerState, id: u64) -> JsonResult {
    PlainWallet::address_index(state, id)
}

#[post(
    "/plain_wallets/<id>/relationships/addresses", data = "<address>"
)]
pub fn create(state: &ServerState, id: u64, address: Address) -> JsonResult {
    PlainWallet::address_create(state, id, address)
}

#[delete(
    "/plain_wallets/<id>/relationships/addresses", data = "<address>"
)]
pub fn destroy(state: &ServerState, id: u64, address: Address) -> JsonResult {
    PlainWallet::address_destroy(state, id, address)
}

#[get(
    "/plain_wallets/relationships/addresses/<address>/balance?<params>"
)]
pub fn balance(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    PlainWallet::balance(&state.executor, address, params.limit, params.since)
}

#[get("/plain_wallets/relationships/addresses/<address>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    PlainWallet::get_utxos(&state.executor, address, params.limit, params.since)
}