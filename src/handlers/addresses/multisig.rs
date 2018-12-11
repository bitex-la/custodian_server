use handlers::addresses::base::AddressHandler;
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

#[get("/multisig_addresses/<address>")]
pub fn show(state: &ServerState, address: String) -> JsonResult {
    MultisigAddress::show(state, address)
}

#[delete("/multisig_addresses/<id>")]
pub fn destroy(state: &ServerState, id: usize) -> JsonResult {
    MultisigAddress::destroy(state, id)
}

#[get("/multisig_addresses/<address>/get_utxos")]
pub fn get_utxos(state: &ServerState, address: String) -> JsonResult {
    MultisigAddress::get_utxos(&state.executor, address, Some(1000000), Some(0))
}
