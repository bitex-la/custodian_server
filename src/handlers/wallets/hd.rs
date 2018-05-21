use handlers::wallets::base;
use models::hd_wallet::HdWallet;
use server_state::ServerState;

#[get("/hd_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> base::JsonResult {
  base::index(state, |wallets| wallets.hds)
}

#[get("/hd_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: i32) -> base::JsonResult {
  base::show(state, id, |wallets| wallets.hds)
}

#[post("/hd_wallets", format = "application/json", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: HdWallet) -> base::JsonResult {
    base::create(state, wallet, |wallets| wallets.hds.as_mut() )
}

#[put("/hd_wallets/<id>", format = "application/json", data = "<wallet>")]
pub fn update(state: &ServerState, id: i32, wallet: HdWallet) -> base::JsonResult {
    base::update(state, id, wallet, |wallets| wallets.hds.as_mut() )
}

#[delete("/hd_wallets/<id>", format = "application/json")]
pub fn destroy(state: &ServerState, id: i32) -> base::JsonResult {
    base::destroy(state, id, |wallets| wallets.hds.as_mut() )
}
