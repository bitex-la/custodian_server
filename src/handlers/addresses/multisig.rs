use handlers::addresses::base::AddressHandler;
use handlers::helpers::GetTransactionParams;
use handlers::helpers::JsonResult;
use models::multisig_address::MultisigAddress;
use server_state::ServerState;
use data_guards::Mapped;

#[get("/multisig_addresses")]
pub fn index(state: &ServerState) -> JsonResult {
    MultisigAddress::index(state)
}

#[post( "/multisig_addresses", data = "<address>")]
pub fn create(state: &ServerState, address: Mapped<MultisigAddress>) -> JsonResult {
    MultisigAddress::create(state, address.0)
}

#[get("/multisig_addresses/<id>")]
pub fn show(state: &ServerState, id: usize) -> JsonResult {
    MultisigAddress::show(state, id)
}

#[delete("/multisig_addresses/<id>")]
pub fn destroy(state: &ServerState, id: usize) -> JsonResult {
    MultisigAddress::destroy(state, id)
}

#[get("/multisig_addresses/<address>/balance?<params>")]
pub fn balance(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    MultisigAddress::balance(&state.executor, address, params.limit, params.since)
}

#[get("/multisig_addresses/<address>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    MultisigAddress::get_utxos(&state.executor, address, params.limit, params.since)
}
