use serde_json;
use serde::ser::Serialize;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::Value;

pub type JsonResult = Result<Json<Value>, status::Custom<String>>;

pub fn to_value<T>(raw_value: T) -> JsonResult 
    where
    T: Serialize
{
    serde_json::to_value(raw_value)
        .map(|value| Json(value))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}
