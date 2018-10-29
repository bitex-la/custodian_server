use handlers::addresses::base::AddressHandler;
use handlers::handler::GetTransactionParams;
use handlers::handler::JsonResult;
use models::multisig_address::MultisigAddress;
use models::resource_address::ResourceAddress;
use server_state::ServerState;

#[get("/multisig_wallets/<id>/relationships/addresses")]
pub fn index(state: &ServerState, id: u64) -> JsonResult {
    MultisigAddress::address_index(state, id)
}

#[post(
    "/multisig_wallets/<id>/relationships/addresses",
    format = "application/json",
    data = "<address>"
)]
pub fn create(state: &ServerState, id: u64, address: ResourceAddress<MultisigAddress>) -> JsonResult {
    MultisigAddress::address_create(state, id, address)
}

#[delete(
    "/multisig_wallets/<id>/relationships/addresses",
    format = "application/json",
    data = "<address>"
)]
pub fn destroy(state: &ServerState, id: u64, address: ResourceAddress<MultisigAddress>) -> JsonResult {
    MultisigAddress::address_destroy(state, id, address)
}

#[get(
    "/multisig_wallets/relationships/addresses/<address>/balance?<params>"
)]
pub fn balance(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    MultisigAddress::balance(&state.executor, address, params.limit, params.since)
}

#[get("/multisig_wallets/relationships/addresses/<address>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    MultisigAddress::get_utxos(&state.executor, address, params.limit, params.since)
}