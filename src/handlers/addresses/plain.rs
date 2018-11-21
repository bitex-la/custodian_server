use handlers::addresses::base::AddressHandler;
use handlers::helpers::JsonResult;
use models::plain_address::PlainAddress;
use server_state::ServerState;
use data_guards::Mapped;

#[get("/plain_addresses")]
pub fn index(state: &ServerState) -> JsonResult {
    PlainAddress::index(state)
}

#[post("/plain_addresses", data = "<address>")]
pub fn create(state: &ServerState, address: Mapped<PlainAddress>) -> JsonResult {
    PlainAddress::create(state, address.0)
}

#[get("/plain_addresses/<id>")]
pub fn show(state: &ServerState, id: usize) -> JsonResult {
    PlainAddress::show(state, id)
}

#[delete("/plain_addresses/<id>")]
pub fn destroy(state: &ServerState, id: usize) -> JsonResult {
    PlainAddress::destroy(state, id)
}

#[get("/plain_addresses/<address>/balance")]
pub fn balance(state: &ServerState, address: String) -> JsonResult {
    PlainAddress::balance(&state.executor, address, Some(1000000), Some(0))
}

#[get("/plain_addresses/<address>/get_utxos")]
pub fn get_utxos(state: &ServerState, address: String) -> JsonResult {
    PlainAddress::get_utxos(&state.executor, address, Some(1000000), Some(0))
}
