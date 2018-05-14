use rocket_contrib::{Json, Value};
use serde_json::to_value;
use jsonapi::model::*;
use rocket::response::status;
use rocket::http::Status;
use server_state::ServerState;
use models::wallets::Wallets;

#[get("/wallets", format = "application/json")]
pub fn index(state: &ServerState) -> Result<Json<Value>, status::Custom<String>> {
    let wallets = state.wallets_lock();
    match to_value(wallets.to_jsonapi_document()) {
        Ok(value) => Ok(Json(value)),
        Err(err)  => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

#[post("/wallets", format = "application/json", data = "<wallets>")]
pub fn create(state: &ServerState, wallets: Wallets) -> Json<Value> {
    let mut state_wallets = state.wallets_lock();
    state_wallets.create(wallets);
    Json(json!({"status": "ok"}))
}

#[put("/wallets", format = "application/json", data = "<wallets>")]
pub fn update(state: &ServerState, wallets: Wallets) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match state_wallets.update(wallets) {
        Ok(_)    => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err))
    }
}
