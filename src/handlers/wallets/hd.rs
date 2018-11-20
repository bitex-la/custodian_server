use handlers::helpers::JsonResult;
use handlers::wallets::base::WalletHandler;
use models::hd_wallet::HdWallet;
use server_state::ServerState;
use data_guards::Mapped;

#[get("/hd_wallets")]
pub fn index(state: &ServerState) -> JsonResult {
    HdWallet::index(state)
}

#[get("/hd_wallets/<id>/get_utxos")]
pub fn get_utxos(state: &ServerState, id: String) -> JsonResult {
    HdWallet::get_utxos(state, id, Some(10000000), Some(0))
}

#[get("/hd_wallets/<id>/get_incoming")]
pub fn get_incoming(state: &ServerState, id: String) -> JsonResult {
    HdWallet::get_incoming(state, id, Some(10000000), Some(0))
}

#[get("/hd_wallets/<id>")]
pub fn show(state: &ServerState, id: String) -> JsonResult {
    HdWallet::show(state, id)
}

#[post("/hd_wallets", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: Mapped<HdWallet>) -> JsonResult {
    HdWallet::create(state, wallet)
}

#[put("/hd_wallets/<id>", data = "<wallet>")]
pub fn update(state: &ServerState, id: String, wallet: Mapped<HdWallet>) -> JsonResult {
    HdWallet::update(state, id, wallet)
}

#[delete("/hd_wallets/<id>")]
pub fn destroy(state: &ServerState, id: String) -> JsonResult {
    HdWallet::destroy(state, id)
}
