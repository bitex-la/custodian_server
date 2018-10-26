use handlers::handler::GetTransactionParams;
use handlers::handler::JsonResult;
use handlers::wallets::base::WalletHandler;
use models::hd_wallet::HdWallet;
use models::resource_wallet::ResourceWallet;
use models::jsonapi_record::JsonApiRecord;
use server_state::ServerState;

#[get("/hd_wallets")]
pub fn index(state: &ServerState) -> JsonResult {
    HdWallet::index(state)
}

#[get("/hd_wallets/<id>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, id: u64, params: GetTransactionParams) -> JsonResult {
    HdWallet::get_utxos(state, id, params.limit, params.since)
}

#[get("/hd_wallets/<id>/get_incoming?<params>")]
pub fn get_incoming(state: &ServerState, id: u64, params: GetTransactionParams) -> JsonResult {
    HdWallet::get_incoming(state, id, params.limit, params.since)
}

#[get("/hd_wallets/<id>")]
pub fn show(state: &ServerState, id: usize) -> JsonResult {
    HdWallet::show(state, id)
}

#[post("/hd_wallets", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: ResourceWallet<HdWallet>) -> JsonResult {
    HdWallet::create(state, wallet)
}

#[put("/hd_wallets/<id>", data = "<wallet>")]
pub fn update(state: &ServerState, id: usize, wallet: ResourceWallet<HdWallet>) -> JsonResult {
    HdWallet::update(state, id, wallet)
}

#[delete("/hd_wallets/<id>")]
pub fn destroy(state: &ServerState, id: usize) -> JsonResult {
    HdWallet::destroy(state, id)
}
