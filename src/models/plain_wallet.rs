use bitprim::executor::Executor;
use jsonapi::model::*;
use models::wallet::Wallet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainWallet {
    pub id: String,
    pub version: String,
    pub addresses: Vec<String>
}

jsonapi_model!(PlainWallet; "plain_wallet");

#[derive(Debug)]
pub struct PlainUtxo {
    pub prev_hash: String,
    pub prev_index: u64,
    pub address: String,
    pub amount: u64
}

impl Wallet for PlainWallet {
    type Utxo = PlainUtxo;

    fn get_utxos(&self, _exec: &Executor) -> Vec<Self::Utxo> {
        vec![
            PlainUtxo {
                prev_hash: "abc".to_string(),
                prev_index: 1,
                address: "abc".to_string(),
                amount: 100000000,
            },
        ]
    }
}

