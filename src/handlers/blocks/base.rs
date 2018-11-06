use handlers::helpers::JsonResult;
use jsonapi::model::*;
use models::block::Block;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::Json;
use serde_json;
use server_state::ServerState;

#[get("/blocks/last")]
pub fn last(state: &ServerState) -> JsonResult {
    let chain = state.executor.get_chain();
    let last_height = chain.get_last_height().unwrap_or(1);
    let bitprim_block = chain
        .get_block_by_height(last_height)
        .expect("Couldn't get last block, check connection");
    let bitprim_header = chain
        .get_block_header_by_height(last_height)
        .expect("Couldn't get last block header, check connection");
    let block = Block {
        id: Some(bitprim_block.0.hash().to_hex()),
        height: last_height,
        timestamp: bitprim_header.0.timestamp(),
    };
    serde_json::to_value(block.to_jsonapi_document())
        .map(|value| Json(value))
        .map_err(|error| status::Custom(Status::InternalServerError, error.to_string()))
}
