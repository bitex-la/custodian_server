use handlers::helpers::JsonResult;
use handlers::wallets::base::{WalletHandler};
use models::plain_wallet::PlainWallet;
use server_state::ServerState;
use data_guards::Mapped;

#[get("/plain_wallets")]
pub fn index(state: &ServerState) -> JsonResult
{
    PlainWallet::index(state)
}

#[get("/plain_wallets/<id>/get_utxos")]
pub fn get_utxos(state: &ServerState, id: String) -> JsonResult {
    PlainWallet::get_utxos(state, id, Some(10000000), Some(0))
}

#[get("/plain_wallets/<id>/get_incoming")]
pub fn get_incoming(state: &ServerState, id: String) -> JsonResult {
    PlainWallet::get_incoming(state, id, Some(10000000), Some(0))
}

#[get("/plain_wallets/<id>")]
pub fn show(state: &ServerState, id: String) -> JsonResult {
    PlainWallet::show(state, id)
}

#[post("/plain_wallets", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: Mapped<PlainWallet>) -> JsonResult {
    PlainWallet::create(state, wallet)
}

#[put("/plain_wallets/<id>", data = "<wallet>")]
pub fn update(state: &ServerState, id: String, wallet: Mapped<PlainWallet>) -> JsonResult {
    PlainWallet::update(state, id, wallet)
}

#[delete("/plain_wallets/<id>")]
pub fn destroy(state: &ServerState, id: String) -> JsonResult {
    PlainWallet::destroy(state, id)
}
