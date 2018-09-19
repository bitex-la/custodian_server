use rocket_contrib::{Json};
use bitprim::hash::Hash;
use server_state::ServerState;

#[post("/transactions/broadcast", format = "application/json", data = "<hash>")]
pub fn broadcast(state: &ServerState, hash: String) -> Json<Hash> {
    let chain = state.executor.get_chain();
    Json(chain.broadcast(&hash))
}
