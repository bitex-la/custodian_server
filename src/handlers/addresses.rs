use rocket_contrib::{Json, Value};
use rocket::response::status;
use server_state::ServerState;
use models::wallets::Wallets;
use models::plain_wallet::Address;

#[post("/plain_wallets/<id>/addresses", format = "application/json", data = "<address>")]
pub fn create(state: &ServerState, id: i32, address: Address) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::add_address(&mut state_wallets.plains, id, address) {
        Ok(_)    => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err))
    }
}

#[delete("/plain_wallets/<id>/addresses", format = "application/json", data = "<address>")]
pub fn destroy(state: &ServerState, id: i32, address: Address) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::destroy_address(&mut state_wallets.plains, id, address) {
        Ok(_)    => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err))
    }
}