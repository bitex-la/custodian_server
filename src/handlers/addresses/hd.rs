use handlers::addresses::base::AddressHandler;
use models::hd_wallet::HdAddress;
use models::hd_wallet::HdWallet;
use server_state::ServerState;
use handlers::handler::JsonResult;

#[get("/hd_wallets/<id>/relationships/addresses", format = "application/json")]
pub fn index(state: &ServerState, id: u64) -> JsonResult {
    HdWallet::address_index(state, id)
}

#[post("/hd_wallets/<id>/relationships/addresses", format = "application/json", data = "<address>")]
pub fn create(state: &ServerState, id: u64, address: HdAddress) -> JsonResult {
    HdWallet::address_create(state, id, address)
}

#[delete("/hd_wallets/<id>/relationships/addresses", format = "application/json", data = "<address>")]
pub fn destroy(state: &ServerState, id: u64, address: HdAddress) -> JsonResult {
    HdWallet::address_destroy(state, id, address)
}
