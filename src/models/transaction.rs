use jsonapi::model::*;

use models::hd_wallet::HdAddress;
use models::plain_wallet::Address;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction<A> {
    pub id: Option<String>,
    pub satoshis: u64,
    pub transaction_hash: String,
    pub position: u32,
    pub is_spent: bool,
    pub block_height: u32,
    pub address: A
}

jsonapi_model!(Transaction<Address>; "plain_transaction"; has one address);
jsonapi_model!(Transaction<HdAddress>; "hd_transaction"; has one address);
