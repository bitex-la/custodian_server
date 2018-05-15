use rocket_contrib::{Json, Value};
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;
use serde_json::to_value;
use jsonapi::model::*;
use models::hd_wallet::HdWallet;
use models::wallets::Wallets;

#[get("/hd_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> Result<Json<Value>, status::Custom<String>> {
    let wallets = state.wallets_lock();
    match to_value(vec_to_jsonapi_document(wallets.clone().hds)) {
        Ok(value) => Ok(Json(value)),
        Err(err)  => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[get("/hd_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: i32) -> Result<Json<Value>, status::NotFound<String>> {
    let state_wallets = state.wallets_lock();
    match Wallets::show_wallet(&state_wallets.hds, id) {
        Ok(wallet) => {
            match to_value(wallet.to_jsonapi_document()) {
                Ok(value) => Ok(Json(value)),
                Err(err)  => Err(status::NotFound(err.to_string())) // Should be InternalServerError, because this is a parser error
            }
        },
        Err(err)   => Err(status::NotFound(err))
    }
}

#[post("/hd_wallets", format = "application/json", data = "<hd_wallet>")]
pub fn create(state: &ServerState, hd_wallet: HdWallet) -> Json<Value> {
    let mut state_wallets = state.wallets_lock();
    state_wallets.hds.push(hd_wallet);
    Json(json!({"status": "ok"}))
}

#[put("/hd_wallets/<id>", format = "application/json", data = "<hd_wallet>")]
pub fn update(state: &ServerState, id: i32, hd_wallet: HdWallet) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::update_wallet(&mut state_wallets.hds, id, hd_wallet) {
        Ok(_)    => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err))
    }
}

#[delete("/hd_wallets/<id>", format = "application/json")]
pub fn destroy(state: &ServerState, id: i32) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::destroy_wallet(&mut state_wallets.hds, id) {
        Ok(_)    => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err))
    }
}

