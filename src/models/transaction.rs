use serde_json;
use bitprim::explorer::Received;
use bitprim::explorer::InputDetail;
use bitprim::explorer::OutputDetail;
use jsonapi::model::*;
use serializers::ToJsonApi;

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

impl ToJsonApi for Transaction {
    const TYPE : &'static str = "transactions";

    fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
        hashmap!{
            "satoshis".to_string() => serde_json::to_value(&self.satoshis).unwrap(),
            "transaction_hash".to_string() => serde_json::to_value(&self.transaction_hash).unwrap(),
            "position".to_string() => serde_json::to_value(&self.position).unwrap(),
            "is_spent".to_string() => serde_json::to_value(&self.is_spent).unwrap(),
            "block_height".to_string() => serde_json::to_value(&self.block_height).unwrap(),
            "address".to_string() => serde_json::to_value(&self.address).unwrap(),
            "version".to_string() => serde_json::to_value(&self.version).unwrap(),
            "locktime".to_string() => serde_json::to_value(&self.locktime).unwrap(),
            "inputs".to_string() => serde_json::to_value(&self.inputs).unwrap(),
            "outputs".to_string() => serde_json::to_value(&self.outputs).unwrap(),
        }
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
