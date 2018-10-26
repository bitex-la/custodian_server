use jsonapi::model::{vec_to_jsonapi_document, JsonApiModel};
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::{Json, Value};
use serde::ser::Serialize;
use serde_json::to_value;
use tiny_ram_db;
use tiny_ram_db::{PlainTable, Record};

pub type JsonResult = Result<Json<Value>, status::Custom<String>>;

#[derive(FromForm)]
pub struct GetTransactionParams {
    pub limit: Option<u64>,
    pub since: Option<u64>,
}

pub fn parse_to_value<T: Serialize>(value: T) -> JsonResult {
    match to_value(value) {
        Ok(value_parsed) => Ok(Json(value_parsed)),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string())),
    }
}

pub fn check_resource_operation<T: Serialize>(
    result_value: Result<T, tiny_ram_db::errors::Error>,
) -> JsonResult {
    match result_value {
        Ok(value) => parse_to_value(value),
        Err(err) => Err(status::Custom(Status::InternalServerError, err.to_string())),
    }
}

pub fn from_record_to_resource_wallet<T: Serialize + Wallet>(
    result_value: Result<Record<T>, tiny_ram_db::errors::Error>,
) -> JsonResult
where
    ResourceWallet<T>: JsonApiModel,
{
    match result_value {
        Ok(record) => {
            let resource_wallet = ResourceWallet {
                id: Some(record.id),
                wallet: (*record.data).clone(),
            };
            parse_to_value(resource_wallet.to_jsonapi_document_with_query(&T::default_query()))
        }
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
