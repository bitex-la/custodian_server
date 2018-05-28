use handlers::addresses::base::AddressHandler;
use handlers::addresses::base::JsonResult;
use models::hd_wallet::HdAddress;
use models::hd_wallet::HdWallet;
use server_state::ServerState;

#[post("/hd_wallets/<id>/addresses", format = "application/json", data = "<address>")]
pub fn create(state: &ServerState, id: u64, address: HdAddress) -> JsonResult {
    HdWallet::address_create(state, id, address)
}

#[delete("/hd_wallets/<id>/addresses", format = "application/json", data = "<address>")]
pub fn destroy(state: &ServerState, id: u64, address: HdAddress) -> JsonResult {
    HdWallet::address_destroy(state, id, address)
}
