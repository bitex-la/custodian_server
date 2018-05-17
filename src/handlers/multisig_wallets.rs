use jsonapi::model::*;
use models::multisig_wallet::MultisigWallet;
use models::wallets::Wallets;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use serde_json::to_value;
use server_state::ServerState;

#[get("/multisig_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> Result<Json<Value>, status::Custom<String>> {
    let wallets = state.wallets_lock();

    match to_value(vec_to_jsonapi_document(wallets.clone().multisigs)) {
        Ok(value) => Ok(Json(value)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string())),
    }
}

#[get("/multisig_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: i32) -> Result<Json<Value>, status::Custom<String>> {
    let state_wallets = state.wallets_lock();

    match Wallets::show_wallet(&state_wallets.multisigs, id) {
        Ok(wallet) => match to_value(wallet.to_jsonapi_document()) {
            Ok(value) => Ok(Json(value)),
            Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string())),
        },
        Err(err) => Err(status::Custom(Status::NotFound, err)),
    }
}

#[post("/multisig_wallets", format = "application/json", data = "<multisig_wallet>")]
pub fn create(state: &ServerState, multisig_wallet: MultisigWallet) -> Json<Value> {
    let mut state_wallets = state.wallets_lock();

    state_wallets.multisigs.push(multisig_wallet);
    Json(json!({"status": "ok"}))
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
