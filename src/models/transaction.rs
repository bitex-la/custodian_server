use bitprim::explorer::Received;
use bitprim::explorer::InputDetail;
use bitprim::explorer::OutputDetail;
use jsonapi::model::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<String>,
    pub satoshis: u64,
    pub transaction_hash: String,
    pub position: u32,
    pub is_spent: bool,
    pub block_height: u32,
    pub address: String,
    pub version: u32,
    pub locktime: u32,
    pub inputs: Vec<InputDetail>,
    pub outputs: Vec<OutputDetail>
}

jsonapi_model!(Transaction; "transaction");

impl Transaction {
    pub fn new(tx: Received, address: String) -> Self {
        let hex_value = tx.transaction_hash.to_hex();
        Transaction {
            id: Some(format!("{}-{}", hex_value, tx.position)),
            satoshis: tx.satoshis,
            transaction_hash: hex_value,
            position: tx.position,
            is_spent: tx.is_spent,
            block_height: tx.block_height,
            address: address,
            version: tx.version,
            locktime: tx.locktime,
            inputs: tx.input_details,
            outputs: tx.output_details
        }
    }
}
