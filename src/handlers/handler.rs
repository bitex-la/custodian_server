use std::ops::Deref;
use tiny_ram_db;
use tiny_ram_db::Record;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use serde::ser::Serialize;
use serde_json::to_value;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;

pub type JsonResult = Result<Json<Value>, status::Custom<String>>;

#[derive(FromForm)]
pub struct GetTransactionParams {
    pub limit: Option<u64>,
    pub since: Option<u64>,
}

pub fn parse_to_value<T: Serialize>(value: T) -> JsonResult {
    match to_value(value) {
        Ok(value_parsed) => Ok(Json(value_parsed)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn check_resource_operation<T: Serialize>(result_value: Result<T, tiny_ram_db::errors::Error>) -> JsonResult {
    match result_value {
        Ok(value) => parse_to_value(value),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}

pub fn from_record_to_resource_wallet<T: Serialize + Wallet>(result_value: Result<Record<T>, tiny_ram_db::errors::Error>) -> JsonResult {
    match result_value {
        Ok(wallet) => {
            let resource = wallet.data.deref();
            parse_to_value(ResourceWallet { id: Some(wallet.id), wallet: resource.clone() })
        },
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string()))
    }
}