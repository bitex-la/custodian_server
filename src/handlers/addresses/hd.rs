use handlers::addresses::base::AddressHandler;
use handlers::helpers::JsonResult;
use models::hd_address::HdAddress;
use server_state::ServerState;
use data_guards::Mapped;

#[get("/hd_addresses")]
pub fn index(state: &ServerState) -> JsonResult {
    HdAddress::index(state)
}

#[post("/hd_addresses", data = "<address>")]
pub fn create(state: &ServerState, address: Mapped<HdAddress>) -> JsonResult {
    HdAddress::create(state, address.0)
}

#[get("/hd_addresses/<address>")]
pub fn show(state: &ServerState, address: String) -> JsonResult {
    HdAddress::show(state, address)
}

#[delete( "/hd_addresses/<id>")]
pub fn destroy(state: &ServerState, id: usize) -> JsonResult {
    HdAddress::destroy(state, id)
}

#[get("/hd_addresses/<address>/get_utxos")]
pub fn get_utxos(state: &ServerState, address: String) -> JsonResult {
    HdAddress::get_utxos(&state.executor, address, Some(1000000), Some(0))
}
