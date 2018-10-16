use handlers::handler::GetTransactionParams;
use handlers::handler::JsonResult;
use handlers::wallets::base::WalletHandler;
use models::hd_wallet::HdWallet;
use server_state::ServerState;

#[get("/hd_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> JsonResult {
    HdWallet::index(state)
}

#[get("/hd_wallets/<id>/get_utxos?<params>", format = "application/json")]
pub fn get_utxos(state: &ServerState, id: u64, params: GetTransactionParams) -> JsonResult {
    HdWallet::get_utxos(state, id, params.limit, params.since)
}

#[get("/hd_wallets/<id>/get_incoming?<params>", format = "application/json")]
pub fn get_incoming(state: &ServerState, id: u64, params: GetTransactionParams) -> JsonResult {
    HdWallet::get_incoming(state, id, params.limit, params.since)
}

#[get("/hd_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: u64) -> JsonResult {
    HdWallet::show(state, id)
}

#[post("/hd_wallets", format = "application/json", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: HdWallet) -> JsonResult {
    HdWallet::create(state, wallet)
}

#[put("/hd_wallets/<id>", format = "application/json", data = "<wallet>")]
pub fn update(state: &ServerState, id: u64, wallet: HdWallet) -> JsonResult {
    HdWallet::update(state, id, wallet)
}

#[delete("/hd_wallets/<id>", format = "application/json")]
pub fn destroy(state: &ServerState, id: u64) -> JsonResult {
    HdWallet::destroy(state, id)
}
