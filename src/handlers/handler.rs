use serde_json;
use jsonapi::model::{vec_to_jsonapi_document, JsonApiModel};
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use serde::ser::Serialize;
use tiny_ram_db;
use tiny_ram_db::{PlainTable};

use models::wallet::Wallet;

pub type JsonResult = Result<Json<Value>, status::Custom<String>>;

#[derive(FromForm)]
pub struct GetTransactionParams {
    pub limit: Option<u64>,
    pub since: Option<u64>,
}

pub fn check_resource_operation<T: Serialize>(
    result_value: Result<T, tiny_ram_db::errors::Error>,
) -> JsonResult {
    let result = result_value
        .map(|value| serde_json::to_value(value))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))?;

    result
        .map(|value| Json(value))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}
