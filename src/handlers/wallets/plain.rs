use handlers::helpers::GetTransactionParams;
use handlers::helpers::JsonResult;
use handlers::wallets::base::{WalletHandler, WalletFilter};
use models::plain_wallet::PlainWallet;
use server_state::ServerState;
use data_guards::Mapped;

#[get("/plain_wallets")]
pub fn index(state: &ServerState) -> JsonResult
{
    PlainWallet::index(state)
}


use rocket::{Request, Route, Data, State, Outcome};
use rocket::handler;
use rocket::http::Method;
use rocket::http::Status;

fn handler<'r>(request: &'r Request, _data: Data) -> handler::Outcome<'r> {
    let state: State<ServerState> = match request.guard::<State<ServerState>>() {
        Outcome::Success(value) => value,
        _ => {
            return Outcome::Failure(Status::BadRequest);
        }
    };
    println!("{:#?}", request.uri().query());
    let wallet_filter = WalletFilter { 
        label: request.uri().query().unwrap().to_string()
    };
    let responder = filter_index(&state, wallet_filter);

    Outcome::from(request, responder)
}

fn filter_index(state: &ServerState, label: WalletFilter) -> JsonResult
{
    PlainWallet::index(state)
}

pub fn index_filter_route() -> Route {
    Route::new(Method::Get, "/plain_wallets?filter[label]=<label>", handler)
}

#[get("/plain_wallets/<id>/get_utxos?<params>")]
pub fn get_utxos(state: &ServerState, id: usize, params: GetTransactionParams) -> JsonResult {
    PlainWallet::get_utxos(state, id, params.limit, params.since)
}

#[get("/plain_wallets/<id>/get_incoming?<params>")]
pub fn get_incoming(state: &ServerState, id: usize, params: GetTransactionParams) -> JsonResult {
    PlainWallet::get_incoming(state, id, params.limit, params.since)
}

#[get("/plain_wallets/<id>")]
pub fn show(state: &ServerState, id: usize) -> JsonResult {
    PlainWallet::show(state, id)
}

#[post("/plain_wallets", data = "<wallet>")]
pub fn create(state: &ServerState, wallet: Mapped<PlainWallet>) -> JsonResult {
    PlainWallet::create(state, wallet)
}

#[put("/plain_wallets/<id>", data = "<wallet>")]
pub fn update(state: &ServerState, id: usize, wallet: Mapped<PlainWallet>) -> JsonResult {
    PlainWallet::update(state, id, wallet)
}

#[delete("/plain_wallets/<id>")]
pub fn destroy(state: &ServerState, id: usize) -> JsonResult {
    PlainWallet::destroy(state, id)
}
