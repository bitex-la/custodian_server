use bitprim::executor::Executor;
use jsonapi::model::*;
use models::wallet::Wallet;
use models::resource_wallet::ResourceWallet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdWallet {
    pub id: String,
    pub version: String,
    pub addresses: Vec<HdAddress>,
    pub xpub: String,
}

jsonapi_model!(HdWallet; "hd_wallet");

pub struct HdUtxo {
    pub prev_hash: String,
    pub prev_index: u64,
    pub address: HdAddress,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdAddress {
    pub id: String,
    pub address: String,
    pub path: Vec<u64>,
}

jsonapi_model!(HdAddress; "hd_address");

impl Wallet for HdWallet {
    type Utxo = HdUtxo;

    fn get_utxos(&self, _exec: &Executor) -> Vec<Self::Utxo> {
        vec![
            HdUtxo {
                prev_hash: "abc".to_string(),
                prev_index: 1,
                address: HdAddress {
                    id: "1".to_string(),
                    address: "abc".to_string(),
                    path: vec![0, 1, 0],
                },
                amount: 100000000,
            },
        ]
    }
}

from_data_wallet!(HdWallet);

impl ResourceWallet for HdWallet {
    fn id(&self) -> i32 {
        self.id.parse::<i32>().unwrap_or(0)
    }

    fn add_address<HdAddress>(&self, address: HdAddress) {
    }
}
