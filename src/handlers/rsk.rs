use handlers::handler::{parse_to_value, JsonResult};
use jsonapi::model::*;
use models::rsk::Rsk;
use server_state::ServerState;

#[get("/rsk/temp_addresses", format = "application/json")]
pub fn temp_addresses() -> JsonResult {
    let rsk = Rsk;
    parse_to_value(rsk.get_temp_addresses())
}
