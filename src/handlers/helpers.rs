use serde_json;
use serde::ser::Serialize;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::{Json, Value};

pub type JsonResult = Result<Json<Value>, status::Custom<String>>;

#[derive(FromForm)]
pub struct GetTransactionParams {
    pub limit: Option<u64>,
    pub since: Option<u64>,
}

pub fn to_value<T>(raw_value: T) -> JsonResult 
    where
    T: Serialize
{
    serde_json::to_value(raw_value)
        .map(|value| Json(value))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}
