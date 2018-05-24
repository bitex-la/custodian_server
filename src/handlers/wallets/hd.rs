use handlers::wallets::base::{JsonResult, WalletHandler};
use models::hd_wallet::HdWallet;
use server_state::ServerState;

#[get("/hd_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> JsonResult {
    HdWallet::index(state)
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
