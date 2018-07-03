use rocket::http::Status;
use rocket::response::status;
use handlers::handler::{JsonResult};
use rocket_contrib::{Json, Value};
use server_state::ServerState;

#[post("/transactions/broadcast", format = "application/json", data = "<hash>")]
pub fn broadcast(state: &ServerState, hash: String) -> JsonResult {
    let chain = state.executor.get_chain();
    match chain.broadcast(&hash) {
        true => Ok(Json(Value::Null)),
        false => Err(status::Custom(Status::InternalServerError, "Error broadcasting transaction".to_string()))
    }
}
