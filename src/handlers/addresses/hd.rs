use handlers::addresses::base::AddressHandler;
use handlers::handler::GetTransactionParams;
use handlers::handler::JsonResult;
use models::hd_wallet::HdAddress;
use models::hd_wallet::HdWallet;
use server_state::ServerState;

#[get("/hd_wallets/<id>/relationships/addresses", format = "application/json")]
pub fn index(state: &ServerState, id: u64) -> JsonResult {
    HdWallet::address_index(state, id)
}

#[post("/hd_wallets/<id>/relationships/addresses", format = "application/json", data = "<address>")]
pub fn create(state: &ServerState, id: u64, address: HdAddress) -> JsonResult {
    HdWallet::address_create(state, id, address)
}

#[delete(
    "/hd_wallets/<id>/relationships/addresses", format = "application/json", data = "<address>"
)]
pub fn destroy(state: &ServerState, id: u64, address: HdAddress) -> JsonResult {
    HdWallet::address_destroy(state, id, address)
}

#[get(
    "/hd_wallets/relationships/addresses/<address>/balance?<params>", format = "application/json"
)]
pub fn balance(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    HdWallet::balance(&state.executor, address, params.limit, params.since)
}

#[get("/hd_wallets/relationships/addresses/<address>/get_utxos?<params>", format = "application/json")]
pub fn get_utxos(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    HdWallet::get_utxos(&state.executor, address, params.limit, params.since)
}