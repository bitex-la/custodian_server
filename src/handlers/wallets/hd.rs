use handlers::helpers::GetTransactionParams;
use handlers::helpers::JsonResult;
use handlers::wallets::base::WalletHandler;
use models::hd_wallet::HdWallet;
use server_state::ServerState;
use data_guards::Mapped;

#[get("/hd_wallets")]
pub fn index(state: &ServerState) -> JsonResult {
    // HdWallet::index(state)
    unimplemented!()
}

#[get("/hd_wallets/<id>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, id: usize, params: GetTransactionParams) -> JsonResult {
    HdWallet::get_utxos(state, id, params.limit, params.since)
}

#[get("/hd_wallets/<id>/get_incoming?<params>")]
pub fn get_incoming(state: &ServerState, id: usize, params: GetTransactionParams) -> JsonResult {
    HdWallet::get_incoming(state, id, params.limit, params.since)
}

#[get("/hd_wallets/<id>")]
pub fn show(state: &ServerState, id: usize) -> JsonResult {
    HdWallet::show(state, id)
}

#[post("/hd_wallets", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: Mapped<HdWallet>) -> JsonResult {
    HdWallet::create(state, wallet)
}

#[put("/hd_wallets/<id>", data = "<wallet>")]
pub fn update(state: &ServerState, id: usize, wallet: Mapped<HdWallet>) -> JsonResult {
    HdWallet::update(state, id, wallet)
}

#[delete("/hd_wallets/<id>")]
pub fn destroy(state: &ServerState, id: usize) -> JsonResult {
    HdWallet::destroy(state, id)
}
