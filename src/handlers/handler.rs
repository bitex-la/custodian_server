use jsonapi::model::{vec_to_jsonapi_document, JsonApiModel};
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use serde::ser::Serialize;
use serde_json::to_value;
use tiny_ram_db;
use tiny_ram_db::{PlainTable, Table, Record};

use models::wallet::Wallet;
use models::address::Address;

pub type JsonResult = Result<Json<Value>, status::Custom<String>>;

#[derive(FromForm)]
pub struct GetTransactionParams {
    pub limit: Option<u64>,
    pub since: Option<u64>,
}

pub fn check_resource_operation<T: Serialize>(
    result_value: Result<T, tiny_ram_db::errors::Error>,
) -> JsonResult {
    match result_value {
        Ok(value) => parse_to_value(value),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string())),
    }
}

pub fn plain_table_to_jsonapi<T>(plain_table: &mut PlainTable<T>) -> JsonResult
where
    T: Wallet,
    ResourceWallet<T>: JsonApiModel,
{
    let result_records = plain_table.data.read();
    match result_records {
        Ok(_records) => {
            let records = _records
                .iter()
                .map(|record| ResourceWallet {
                    id: Some(record.id),
                    wallet: (*record.data).clone(),
                })
                .collect::<Vec<ResourceWallet<T>>>();
            parse_to_value(vec_to_jsonapi_document(records))
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error Accesing Database".to_string(),
        )),
    }
}
