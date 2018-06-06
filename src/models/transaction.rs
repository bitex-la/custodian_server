use jsonapi::model::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<String>,
    pub satoshis: u64,
    pub transaction_hash: String,
    pub position: u32,
    pub is_spent: bool,
    pub block_height: u32,
    pub address: String,
}

jsonapi_model!(Transaction; "transaction");
