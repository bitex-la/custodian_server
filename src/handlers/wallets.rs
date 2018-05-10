use rocket_contrib::{Json, Value};
use serde_json::to_value;
use jsonapi::model::*;
use rocket::response::status;
use rocket::http::Status;
use server_state::ServerState;
use wallet::Wallets;

#[get("/wallets")]
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
    *state_wallets = wallets;
    Json(json!({"status": "ok"}))
}
