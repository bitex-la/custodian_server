use handlers::wallets::base;
use models::hd_wallet::HdWallet;
use models::wallets::Wallets;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use server_state::ServerState;

#[get("/hd_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> base::JsonResult {
  base::index(state, |wallets| wallets.hds)
}

#[get("/hd_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: i32) -> base::JsonResult {
  base::show(state, id, |wallets| wallets.hds)
}

#[post("/hd_wallets", format = "application/json", data = "<hd_wallet>")]
pub fn create(state: &ServerState, plain_wallet: PlainWallet) -> base::JsonResult {
  base::create(state, plain_wallet, |wallets| wallets.hds.as_mut() )
}

#[put("/hd_wallets/<id>", format = "application/json", data = "<hd_wallet>")]
pub fn update(
    state: &ServerState,
    id: i32,
    hd_wallet: HdWallet,
) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::update_wallet(&mut state_wallets.hds, id, hd_wallet) {
        Ok(_) => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err)),
    }
}

#[delete("/hd_wallets/<id>", format = "application/json")]
pub fn destroy(state: &ServerState, id: i32) -> Result<Json<Value>, status::NotFound<String>> {
    let mut state_wallets = state.wallets_lock();

    match Wallets::destroy_wallet(&mut state_wallets.hds, id) {
        Ok(_) => Ok(Json(json!({"status": "ok"}))),
        Err(err) => Err(status::NotFound(err)),
    }
}
