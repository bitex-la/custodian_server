use handlers::addresses::base::AddressHandler;
use handlers::helpers::GetTransactionParams;
use handlers::helpers::JsonResult;
use models::hd_address::HdAddress;
use server_state::ServerState;
use handlers::addresses::base::AddressFilters;
use data_guards::Mapped;

#[get("/hd_addresses?<filters>")]
pub fn index(state: &ServerState, filters: AddressFilters) -> JsonResult {
    HdAddress::index(state, filters)
}

#[post("/hd_addresses", data = "<address>")]
pub fn create(state: &ServerState, address: Mapped<HdAddress>) -> JsonResult {
    HdAddress::create(state, address.0)
}

#[get("/hd_addresses/<id>")]
pub fn show(state: &ServerState, id: usize) -> JsonResult {
    HdAddress::show(state, id)
}

#[delete( "/hd_addresses/<id>")]
pub fn destroy(state: &ServerState, id: usize) -> JsonResult {
    HdAddress::destroy(state, id)
}

#[get("/hd_addresses/<address>/balance?<params>")]
pub fn balance(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    HdAddress::balance(&state.executor, address, params.limit, params.since)
}

#[get("/hd_addresses/<address>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    HdAddress::get_utxos(&state.executor, address, params.limit, params.since)
}
