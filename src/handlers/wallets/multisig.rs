use handlers::wallets::base;
use models::multisig_wallet::MultisigWallet;
use server_state::ServerState;

#[get("/multisig_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> base::JsonResult {
  base::index(state, |wallets| wallets.multisigs)
}

#[get("/multisig_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: i32) -> base::JsonResult {
  base::show(state, id, |wallets| wallets.multisigs)
}

#[post("/multisig_wallets", format = "application/json", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: MultisigWallet) -> base::JsonResult {
  base::create(state, wallet, |wallets| wallets.multisigs.as_mut() )
}

#[put("/multisig_wallets/<id>", format = "application/json", data = "<wallet>")]
pub fn update(state: &ServerState, id: i32, wallet: MultisigWallet) -> base::JsonResult {
    base::update(state, id, wallet, |wallets| wallets.multisigs.as_mut() )
}

#[delete("/multisig_wallets/<id>", format = "application/json")]
pub fn destroy(state: &ServerState, id: i32) -> base::JsonResult {
    base::destroy(state, id, |wallets| wallets.multisigs.as_mut() )
}
