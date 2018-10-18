use handlers::handler::GetTransactionParams;
use handlers::handler::JsonResult;
use handlers::wallets::base::WalletHandler;
use models::multisig_wallet::MultisigWallet;
use server_state::ServerState;

#[get("/multisig_wallets")]
pub fn index(state: &ServerState) -> JsonResult {
    MultisigWallet::index(state)
}

#[get("/multisig_wallets/<id>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, id: u64, params: GetTransactionParams) -> JsonResult {
    MultisigWallet::get_utxos(state, id, params.limit, params.since)
}

#[get("/multisig_wallets/<id>/get_incoming?<params>")]
pub fn get_incoming(state: &ServerState, id: u64, params: GetTransactionParams) -> JsonResult {
    MultisigWallet::get_incoming(state, id, params.limit, params.since)
}

#[get("/multisig_wallets/<id>")]
pub fn show(state: &ServerState, id: u64) -> JsonResult {
    MultisigWallet::show(state, id)
}

#[post("/multisig_wallets", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: MultisigWallet) -> JsonResult {
    MultisigWallet::create(state, wallet)
}

#[put("/multisig_wallets/<id>", data = "<wallet>")]
pub fn update(state: &ServerState, id: u64, wallet: MultisigWallet) -> JsonResult {
    MultisigWallet::update(state, id, wallet)
}

#[delete("/multisig_wallets/<id>")]
pub fn destroy(state: &ServerState, id: u64) -> JsonResult {
    MultisigWallet::destroy(state, id)
}
