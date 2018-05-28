use rocket_contrib::{Json, Value};
use rocket::response::status;
use rocket::http::Status;
use serde_json::to_value;
use serde::ser::Serialize;

pub type JsonResult = Result<Json<Value>, status::Custom<String>>;

pub fn parse_to_value<T: Serialize>(value: T) -> JsonResult {
    match to_value(value) {
        Ok(value_parsed) => Ok(Json(value_parsed)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string())),
    }
}
