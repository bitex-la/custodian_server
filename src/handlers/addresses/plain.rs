use handlers::addresses::base::AddressHandler;
use handlers::helpers::GetTransactionParams;
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

#[get("/plain_addresses/<address>/balance?<params>")]
pub fn balance(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    PlainAddress::balance(&state.executor, address, params.limit, params.since)
}

#[get("/plain_addresses/<address>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    PlainAddress::get_utxos(&state.executor, address, params.limit, params.since)
}
