use handlers::wallets::base::{WalletHandler, JsonResult};
use models::plain_wallet::{PlainWallet};
use server_state::ServerState;

#[get("/plain_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> JsonResult {
    PlainWallet::index(state)
}

#[get("/plain_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: u64) -> JsonResult {
    PlainWallet::show(state, id)
}

#[post("/plain_wallets", format = "application/json", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: PlainWallet) -> JsonResult {
    PlainWallet::create(state, wallet)
}

#[put("/plain_wallets/<id>", format = "application/json", data = "<wallet>")]
pub fn update(state: &ServerState, id: u64, wallet: PlainWallet) -> JsonResult {
    PlainWallet::update(state, id, wallet)
}

#[delete("/plain_wallets/<id>", format = "application/json")]
pub fn destroy(state: &ServerState, id: u64) -> JsonResult {
    PlainWallet::destroy(state, id)
}
