use handlers::handler::GetTransactionParams;
use handlers::handler::JsonResult;
use handlers::wallets::base::WalletHandler;
use models::plain_wallet::PlainWallet;
use models::resource_wallet::ResourceWallet;
use models::jsonapi_record::JsonApiRecord;
use server_state::ServerState;

#[get("/plain_wallets")]
pub fn index(state: &ServerState) -> JsonResult {
    PlainWallet::index(state)
}

#[get("/plain_wallets/<id>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, id: u64, params: GetTransactionParams) -> JsonResult {
    PlainWallet::get_utxos(state, id, params.limit, params.since)
}

#[get("/plain_wallets/<id>/get_incoming?<params>")]
pub fn get_incoming(state: &ServerState, id: u64, params: GetTransactionParams) -> JsonResult {
    PlainWallet::get_incoming(state, id, params.limit, params.since)
}

#[get("/plain_wallets/<id>")]
pub fn show(state: &ServerState, id: usize) -> JsonResult {
    PlainWallet::show(state, id)
}

#[post("/plain_wallets", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: ResourceWallet<PlainWallet>) -> JsonResult {
    PlainWallet::create(state, wallet)
}

#[put("/plain_wallets/<id>", data = "<wallet>")]
pub fn update(state: &ServerState, id: u64, wallet: ResourceWallet<PlainWallet>) -> JsonResult {
    PlainWallet::update(state, id, wallet)
}

#[delete("/plain_wallets/<id>")]
pub fn destroy(state: &ServerState, id: u64) -> JsonResult {
    PlainWallet::destroy(state, id)
}
