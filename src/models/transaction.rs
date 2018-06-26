use bitprim::explorer::Received;
use bitprim::executor::Executor;
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
    pub version: u32,
    pub locktime: u32
}

jsonapi_model!(Transaction; "transaction");

impl Transaction {
    pub fn new(exec: &Executor, tx: Received, address: String) -> Self {
        let transaction = tx.get_transaction(exec);
        let hex_value = transaction.hash().to_hex();
        Transaction {
            id: Some(format!("{}-{}", hex_value, tx.position)),
            satoshis: tx.satoshis,
            transaction_hash: hex_value,
            position: tx.position,
            is_spent: tx.is_spent,
            block_height: tx.block_height,
            address: address,
            version: transaction.version(),
            locktime: transaction.locktime()
        }
    }
}
