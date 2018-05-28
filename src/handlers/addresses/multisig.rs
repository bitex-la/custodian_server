use handlers::addresses::base::AddressHandler;
use handlers::handler::JsonResult;
use models::multisig_wallet::HdAddress;
use models::multisig_wallet::MultisigWallet;
use server_state::ServerState;

#[get("/multisig_wallets/<id>/relationships/addresses", format = "application/json")]
pub fn index(state: &ServerState, id: u64) -> JsonResult {
    MultisigWallet::address_index(state, id)
}

#[post(
    "/multisig_wallets/<id>/relationships/addresses",
    format = "application/json",
    data = "<address>"
)]
pub fn create(state: &ServerState, id: u64, address: HdAddress) -> JsonResult {
    MultisigWallet::address_create(state, id, address)
}

#[delete(
    "/multisig_wallets/<id>/relationships/addresses",
    format = "application/json",
    data = "<address>"
)]
pub fn destroy(state: &ServerState, id: u64, address: HdAddress) -> JsonResult {
    MultisigWallet::address_destroy(state, id, address)
}
