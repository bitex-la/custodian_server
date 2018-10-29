use handlers::addresses::base::AddressHandler;
use handlers::handler::GetTransactionParams;
use handlers::handler::JsonResult;
use models::plain_address::PlainAddress;
use models::resource_address::ResourceAddress;
use server_state::ServerState;
use handlers::addresses::base::AddressFilters;

#[get("/plain_addresses?<filters>")]
pub fn index(state: &ServerState, filters: AddressFilters) -> JsonResult {
    PlainAddress::index(state, filters)
}

#[post("/plain_addresses", data = "<address>")]
pub fn create(state: &ServerState, address: ResourceAddress<PlainAddress>) -> JsonResult {
    PlainAddress::create(state, address)
}

#[get("/plain_addresses/<id>")]
pub fn show(state: &ServerState, id: usize) -> JsonResult {
    PlainAddress::show(state, id)
}

#[delete(
    "/plain_wallets/<id>/relationships/addresses", data = "<address>"
)]
pub fn destroy(state: &ServerState, id: u64, address: ResourceAddress<PlainAddress>) -> JsonResult {
    PlainAddress::address_destroy(state, id, address)
}

#[get(
    "/plain_wallets/relationships/addresses/<address>/balance?<params>"
)]
pub fn balance(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    PlainAddress::balance(&state.executor, address, params.limit, params.since)
}

#[get("/plain_wallets/relationships/addresses/<address>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    PlainAddress::get_utxos(&state.executor, address, params.limit, params.since)
}