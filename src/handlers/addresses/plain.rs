use handlers::addresses::base::AddressHandler;
use handlers::addresses::base::JsonResult;
use models::plain_wallet::Address;
use models::plain_wallet::PlainWallet;
use server_state::ServerState;

#[post("/plain_wallets/<id>/addresses", format = "application/json", data = "<address>")]
pub fn create(state: &ServerState, id: u64, address: Address) -> JsonResult {
    PlainWallet::create(state, id, address)
}

#[delete("/plain_wallets/<id>/addresses", format = "application/json", data = "<address>")]
pub fn destroy(state: &ServerState, id: u64, address: Address) -> JsonResult {
    PlainWallet::destroy(state, id, address)
}
