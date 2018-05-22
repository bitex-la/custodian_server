use models::hd_wallet::HdAddress;
use models::wallets::Wallets;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use server_state::ServerState;

#[post("/multisig_wallets/<id>/addresses", format = "application/json", data = "<address>")]
pub fn create(
    state: &ServerState,
    id: u64,
    address: HdAddress,
) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::add_address(&mut state_wallets.multisigs, id, address) {
        Ok(_) => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err)),
    }
}

#[delete("/multisig_wallets/<id>/addresses", format = "application/json", data = "<address>")]
pub fn destroy(
    state: &ServerState,
    id: u64,
    address: HdAddress,
) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::destroy_address(&mut state_wallets.multisigs, id, address) {
        Ok(_) => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err)),
    }
}
