use handlers::wallets::base::{JsonResult, WalletHandler};
use models::multisig_wallet::MultisigWallet;
use server_state::ServerState;

#[get("/multisig_wallets", format = "application/json")]
pub fn index(state: &ServerState) -> JsonResult {
    MultisigWallet::index(state)
}

#[get("/multisig_wallets/<id>", format = "application/json")]
pub fn show(state: &ServerState, id: u64) -> JsonResult {
    MultisigWallet::show(state, id)
}

#[post("/multisig_wallets", format = "application/json", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: MultisigWallet) -> JsonResult {
    MultisigWallet::create(state, wallet)
}

#[put("/multisig_wallets/<id>", format = "application/json", data = "<wallet>")]
pub fn update(state: &ServerState, id: u64, wallet: MultisigWallet) -> JsonResult {
    MultisigWallet::update(state, id, wallet)
}

#[delete("/multisig_wallets/<id>", format = "application/json")]
pub fn destroy(state: &ServerState, id: u64) -> JsonResult {
    MultisigWallet::destroy(state, id)
}
