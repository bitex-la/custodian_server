use jsonapi::model::*;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use server_state::ServerState;
use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallets::Wallets;

pub type JsonResult = Result<Json<Value>, status::Custom<String>>;

pub fn create<A, L, W>(
    state: &ServerState,
    id: i32,
    address: A,
    lambda: L
    ) -> JsonResult
where
    for<'a> L: FnOnce(&'a mut Wallets) -> &'a mut Vec<W>,
    W: JsonApiModel + ResourceWallet<A>,
    A: ResourceAddress
{
    let mut wallets = state.wallets_lock();
    let haystack = lambda(&mut wallets);

    let index = haystack.iter().position(|wallet| wallet.id() == id);
    match index {
        Some(value) => { 
            haystack[value].add_address(address);
            Ok(Json(json!({"status": "ok"})))
        },
        None => Err(status::Custom(Status::NotFound, format!("Wallet {:?} Not Found", id))),
    }
}
