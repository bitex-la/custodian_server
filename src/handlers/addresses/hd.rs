use handlers::addresses::base::AddressHandler;
use handlers::handler::GetTransactionParams;
use handlers::handler::JsonResult;
use models::hd_address::HdAddress;
use models::resource_address::ResourceAddress;
use server_state::ServerState;
use handlers::addresses::base::AddressFilters;

#[get("/hd_addresses?<filters>")]
pub fn index(state: &ServerState, filters: AddressFilters) -> JsonResult {
    HdAddress::index(state, filters)
}

#[post("/hd_wallets/<id>/relationships/addresses", data = "<address>")]
pub fn create(state: &ServerState, id: u64, address: ResourceAddress<HdAddress>) -> JsonResult {
    HdAddress::address_create(state, id, address)
}

#[delete(
    "/hd_wallets/<id>/relationships/addresses", data = "<address>"
)]
pub fn destroy(state: &ServerState, id: u64, address: ResourceAddress<HdAddress>) -> JsonResult {
    HdAddress::address_destroy(state, id, address)
}

#[get(
    "/hd_wallets/relationships/addresses/<address>/balance?<params>"
)]
pub fn balance(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    HdAddress::balance(&state.executor, address, params.limit, params.since)
}

#[get("/hd_wallets/relationships/addresses/<address>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    HdAddress::get_utxos(&state.executor, address, params.limit, params.since)
}