use bitprim::explorer::Received;
use bitprim::chain::Chain;
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
    pub fn new(tx: Received, chain: Chain, address: String) -> Self {
        let hash = tx.transaction_hash;
        let hex_value = hash.to_hex();
        let raw_transaction = chain.get_transaction(hash, 0).expect("Problem connecting with Node to get Transaction");
        Transaction {
            id: Some(format!("{}-{}", hex_value, tx.position)),
            satoshis: tx.satoshis,
            transaction_hash: hex_value,
            position: tx.position,
            is_spent: tx.is_spent,
            block_height: tx.block_height,
            address: address,
            version: raw_transaction.0.version(),
            locktime: raw_transaction.0.locktime()
        }
    }
}
