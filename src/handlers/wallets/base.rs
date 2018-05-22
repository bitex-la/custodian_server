use jsonapi::model::*;
use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallets::Wallets;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use serde_json::to_value;
use server_state::ServerState;

pub type JsonResult = Result<Json<Value>, status::Custom<String>>;

pub fn index<L, J>(state: &ServerState, lambda: L) -> JsonResult
where
    L: FnOnce(Wallets) -> Vec<J>,
    J: JsonApiModel,
{
    let wallets = state.wallets_lock();
    let wallet = lambda(wallets.clone());

    match to_value(vec_to_jsonapi_document(wallet)) {
        Ok(value) => Ok(Json(value)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string())),
    }
}

pub fn show<L, W, A>(state: &ServerState, id: i32, lambda: L) -> JsonResult
  where L: FnOnce(Wallets) -> Vec<W>,
        W: JsonApiModel + ResourceWallet<A>,
        A: ResourceAddress
{
    let wallets = state.wallets_lock();
    let haystack = lambda(wallets.clone());
    let maybe_wallet = &haystack.iter().find(|&wallet| wallet.id() == id);

    match maybe_wallet {
        Some(wallet) => match to_value(wallet.to_jsonapi_document()) {
            Ok(value) => Ok(Json(value)),
            Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string())),
        },
        None => Err(status::Custom(Status::NotFound, format!("{:?}", id))),
    }
}

pub fn create<L, W, A>(state: &ServerState, new: W, lambda: L) -> JsonResult
where
    for<'a> L: FnOnce(&'a mut Wallets) -> &'a mut Vec<W>,
    W: JsonApiModel + ResourceWallet<A>,
    A: ResourceAddress,
{
    let mut wallets = state.wallets_lock();
    let haystack = lambda(&mut wallets);

    if haystack
        .iter()
        .find(|&wallet| wallet.id() == new.id())
        .is_some()
    {
        Err(status::Custom(
            Status::InternalServerError,
            format!("Wallet with id {:?} is duplicated", new.id()),
        ))
    } else {
        haystack.push(new);
        Ok(Json(json!({"status": "ok"})))
    }
} 

pub fn update<L,W,A>(state: &ServerState, id: i32, new: W,  lambda: L) -> JsonResult
  where for<'a> L: FnOnce(&'a mut Wallets) -> &'a mut Vec<W>,
        W: JsonApiModel + ResourceWallet<A>,
        A: ResourceAddress
{
    let mut wallets = state.wallets_lock();
    let haystack = lambda(&mut wallets);
    let maybe_position = &haystack.iter().position(|ref wallet| wallet.id() == id);

    match maybe_position {
        Some(position) => {
            let old_item = haystack.swap_remove(*position);
            let new_item = old_item.merge(new);
            haystack.push(new_item);
            Ok(Json(to_value(&haystack.last()).expect("Serialize after update")))
        },
        None => Err(status::Custom(Status::NotFound, format!("{:?}", id)))
    }
}

pub fn destroy<L, W, A>(state: &ServerState, id: i32, lambda: L) -> JsonResult
  where for<'a> L: FnOnce(&'a mut Wallets) -> &'a mut Vec<W>,
        W: JsonApiModel + ResourceWallet<A>,
        A: ResourceAddress
{
    let mut wallets = state.wallets_lock();
    let haystack = lambda(&mut wallets);
    let maybe_position = &haystack.iter().position(|ref wallet| wallet.id() == id);

    match maybe_position {
        Some(position) => {
            let old = haystack.swap_remove(*position);
            Ok(Json(to_value(&old).expect("Serialize after destroy")))
        },
        None => Err(status::Custom(Status::NotFound, format!("{:?}", id)))
    }
}
