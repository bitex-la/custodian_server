use bitprim::explorer::Received;
use bitprim::explorer::InputDetail;
use bitprim::explorer::OutputDetail;
use models::resource_transaction::JsonApiModelTransaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
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

impl JsonApiModelTransaction  for Transaction {
    fn jsonapi_type() -> &'static str {
        "transaction"
    }
}

impl Transaction {
    pub fn new(tx: Received, address: String) -> Self {
        let hex_value = tx.transaction_hash.to_hex();
        Transaction {
            satoshis: tx.satoshis,
            transaction_hash: hex_value,
            position: tx.position,
            is_spent: tx.is_spent,
            block_height: tx.block_height,
            address,
            version: tx.version,
            locktime: tx.locktime,
            inputs: tx.input_details,
            outputs: tx.output_details
        }
    }
}
