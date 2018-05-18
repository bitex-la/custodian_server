use handlers::wallets::base;
use models::plain_wallet::PlainWallet;
use models::wallets::Wallets;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use server_state::ServerState;

#[get("/plain_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> base::JsonResult {
  base::index(state, |wallets| wallets.plains)
}

#[get("/plain_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: i32) -> base::JsonResult {
  base::show(state, id, |wallets| wallets.plains)
}

#[post("/plain_wallets", format = "application/json", data = "<plain_wallet>")]
pub fn create(state: &ServerState, plain_wallet: PlainWallet) -> base::JsonResult {
  base::create(state, plain_wallet, |wallets| wallets.plains.as_mut() )
}

#[put("/plain_wallets/<id>", format = "application/json", data = "<plain_wallet>")]
pub fn update(
    state: &ServerState,
    id: i32,
    plain_wallet: PlainWallet,
) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::update_wallet(&mut state_wallets.plains, id, plain_wallet) {
        Ok(_) => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err)),
    }
}

#[delete("/plain_wallets/<id>", format = "application/json")]
pub fn destroy(state: &ServerState, id: i32) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::destroy_wallet(&mut state_wallets.plains, id) {
        Ok(_) => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err)),
    }
}
