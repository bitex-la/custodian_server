use rocket_contrib::{Json};
use server_state::ServerState;
use wallet::PlainWallet;
use std::ops::Deref;

#[get("/plain_wallets")]
pub fn index(state: &ServerState) -> Json<Vec<PlainWallet>> {
    let wallets = state.wallets_lock();
    Json(wallets.deref().clone().plain_wallets)
}
