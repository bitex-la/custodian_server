use models::plain_wallet::Address;
use models::plain_wallet::PlainWallet;
use server_state::ServerState;
use handlers::addresses::base::AddressHandler;
use handlers::addresses::base::JsonResult;

#[post("/plain_wallets/<id>/addresses", format = "application/json", data = "<address>")]
pub fn create(state: &ServerState, id: u64, address: Address) -> JsonResult {
    PlainWallet::create(state, id, address)
}

/*#[delete("/plain_wallets/<id>/addresses", format = "application/json", data = "<address>")]
pub fn destroy(
    state: &ServerState,
    id: u64,
    address: Address,
) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::destroy_address(&mut state_wallets.plains, id, address) {
        Ok(_) => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err)),
    }
}
*/
