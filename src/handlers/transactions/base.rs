use rocket::http::Status;
use rocket::response::status;
use handlers::handler::{parse_to_value, JsonResult};
use rocket_contrib::{Json, Value};
use jsonapi::model::*;
use bitprim::hash::Hash;
use server_state::ServerState;

#[post("/transactions/broadcast", format = "application/json", data = "<hash>")]
pub fn broadcast(state: &ServerState, hash: String) -> JsonResult {
    let chain = state.executor.get_chain();
    match chain.broadcast(&hash) {
        Ok(_) => Ok(Json(Value::Null)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}
