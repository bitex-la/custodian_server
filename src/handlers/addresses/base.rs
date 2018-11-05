use std::str::FromStr;

use bitprim::executor::Executor;
use bitprim::payment_address::PaymentAddress;
use handlers::handler::JsonResult;
use jsonapi::model::*;
use models::address::Address;
use models::transaction::Transaction;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;
use tiny_ram_db;

#[derive(FromForm, Debug)]
pub struct AddressFilters {
    pub wallet_id: Option<usize>,
}

/* This trait is the base of all the address handlers, it should only
 * take care of receiving the request input like filters and fields,
 * and serializing the output to jsonapi.
 */
pub trait AddressHandler
where
    Self: serde::Serialize + Address,
    <Self as Address>::Index: tiny_ram_db::Indexer<Item = Self>,
{
    fn index(state: &ServerState, filters: AddressFilters) -> JsonResult {
        let db = state.database_lock();
        let addresses = if let Some(wallet_id) = filters.wallet_id {
            Self::filter_by_wallet(wallet_id, &db)
                .map_err(|_| status::NotFound("Wallet not found") )?;
        } else {
            Self::addresses_from_database(&db);
        }
        vec_to_jsonapi_document(addresses);
    }

    fn create(state: &ServerState, new: Self) -> JsonResult {
        let mut database = state.database_lock();
        let addresses = Self::addresses_from_database(&mut database);

        check_resource_operation(addresses.insert(new))
    }

    fn show(state: &ServerState, id: usize) -> JsonResult {
        let mut database = state.database_lock();
        let addresses = Self::addresses_from_database(&mut database);

        from_record_to_resource_address(addresses.find(id))
    }

    //TODO: Naive version
    fn destroy(state: &ServerState, id: usize) -> JsonResult {
        let mut database = state.database_lock();
        let addresses = Self::addresses_from_database(&mut database);

        let mut vec_records = addresses.data.write().unwrap();
        vec_records.remove(id);

        parse_to_value(true)
    }

    fn balance(
        exec: &Executor,
        address: String,
        limit: Option<u64>,
        since: Option<u64>,
    ) -> JsonResult {
        let explorer = exec.explorer();

        if let Ok(valid_address) = PaymentAddress::from_str(&address) {
            match explorer.address_unspents(
                valid_address,
                limit.unwrap_or(10_000),
                since.unwrap_or(0),
            ) {
                Ok(vec_received) => {
                    parse_to_value(vec_received.iter().map(|r| r.satoshis).sum::<u64>())
                }
                Err(error) => Err(status::Custom(
                    Status::InternalServerError,
                    error.to_string(),
                )),
            }
        } else {
            Err(status::Custom(
                Status::InternalServerError,
                "Invalid Address".to_string(),
            ))
        }
    }

    fn get_utxos(
        exec: &Executor,
        address: String,
        limit: Option<u64>,
        since: Option<u64>,
    ) -> JsonResult {
        let explorer = exec.explorer();

        if let Ok(valid_address) = PaymentAddress::from_str(&address) {
            match explorer.address_unspents(
                valid_address,
                limit.unwrap_or(10_000),
                since.unwrap_or(0),
            ) {
                Ok(vec_received) => parse_to_value(
                    vec_received
                        .into_iter()
                        .map(|received| Transaction::new(received, address.clone()))
                        .collect::<Vec<Transaction>>(),
                ),
                Err(error) => Err(status::Custom(
                    Status::InternalServerError,
                    error.to_string(),
                )),
            }
        } else {
            Err(status::Custom(
                Status::InternalServerError,
                "Invalid Address".to_string(),
            ))
        }
    }
}

impl<R> AddressHandler for R
where
    R: serde::Serialize + Address,
    <R as Address>::Index: tiny_ram_db::Indexer<Item = Self>,
{
}
