use handlers::wallets::base;
use models::plain_wallet::PlainWallet;
use server_state::ServerState;

#[get("/plain_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> base::JsonResult {
    base::index(state, |wallets| wallets.plains)
}

#[get("/plain_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: u64) -> base::JsonResult {
    base::show(state, id, |wallets| wallets.plains)
}

#[post("/plain_wallets", format = "application/json", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: PlainWallet) -> base::JsonResult {
    base::create(state, wallet, |wallets| wallets.plains.as_mut() )
}

#[put("/plain_wallets/<id>", format = "application/json", data = "<wallet>")]
pub fn update(state: &ServerState, id: u64, wallet: PlainWallet) -> base::JsonResult {
    base::update(state, id, wallet, |wallets| wallets.plains.as_mut() )
}

#[delete("/plain_wallets/<id>", format = "application/json")]
pub fn destroy(state: &ServerState, id: u64) -> base::JsonResult {
    base::destroy(state, id, |wallets| wallets.plains.as_mut() )
}
