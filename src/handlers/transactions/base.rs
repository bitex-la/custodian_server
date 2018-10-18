use rocket_contrib::{Json};
use server_state::ServerState;

#[post("/transactions/broadcast", data = "<hash>")]
pub fn broadcast(state: &ServerState, hash: String) -> Json<String> {
    let chain = state.executor.get_chain();
    Json(chain.broadcast(&hash).to_hex())
}
