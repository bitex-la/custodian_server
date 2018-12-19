use std::io::Read;
use rocket::Data;
use rocket_contrib::json::Json;
use server_state::ServerState;

#[post("/transactions/broadcast", data = "<data>")]
pub fn broadcast(state: &ServerState, data: Data) -> Json<String> {
    let mut stream = data.open().take(256);
    let mut hash = String::new();
    if let Err(_) = stream.read_to_string(&mut hash) {
        return Json("Error parsing hex to broadcast".to_string())
    }

    let chain = state.executor.get_chain();
    Json(chain.broadcast(&hash).to_hex())
}
