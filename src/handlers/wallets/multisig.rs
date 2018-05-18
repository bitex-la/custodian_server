use handlers::wallets::base;
use models::multisig_wallet::MultisigWallet;
use models::wallets::Wallets;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use server_state::ServerState;

#[get("/multisig_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> base::JsonResult {
  base::index(state, |wallets| wallets.multisigs)
}

#[get("/multisig_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: i32) -> base::JsonResult {
  base::show(state, id, |wallets| wallets.multisigs)
}

#[post("/multisig_wallets", format = "application/json", data = "<multisig_wallet>")]
pub fn create(state: &ServerState, multisig_wallet: MultisigWallet) -> Result<Json<Value>, status::Custom<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::add_wallet(&mut state_wallets.multisigs, multisig_wallet) {
        Ok(_)    => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[put("/multisig_wallets/<id>", format = "application/json", data = "<multisig_wallet>")]
pub fn update(
    state: &ServerState,
    id: i32,
    multisig_wallet: MultisigWallet,
) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::update_wallet(&mut state_wallets.multisigs, id, multisig_wallet) {
        Ok(_) => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err)),
    }
}

#[delete("/multisig_wallets/<id>", format = "application/json")]
pub fn destroy(state: &ServerState, id: i32) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::destroy_wallet(&mut state_wallets.multisigs, id) {
        Ok(_) => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err)),
    }
}
