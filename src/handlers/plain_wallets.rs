use rocket_contrib::{Json, Value};
use server_state::ServerState;
use wallet::PlainWallet;

#[get("/plain_wallets")]
pub fn index(state: &ServerState) -> Json<Vec<PlainWallet>> {
    let wallets = state.wallets_lock();
    Json(wallets.plain_wallets.clone())
}

#[post("/plain_wallets", format = "application/json", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: Json<PlainWallet>) -> Json<Value> {
    let mut wallets = state.wallets_lock();
    wallets.plain_wallets.push(wallet.into_inner());
    Json(json!({"status": "ok"}))
}
