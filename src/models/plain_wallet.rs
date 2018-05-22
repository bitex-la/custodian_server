use std::io::Read;

use bitprim::executor::Executor;
use jsonapi::model::*;
use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
  pub id: Option<String>
}
impl ResourceAddress for Address {}
jsonapi_model!(Address; "address");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainWallet {
    pub id: Option<String>,
    pub version: String,
    #[serde(default)]
    pub addresses: Vec<Address>,
}

jsonapi_model!(PlainWallet; "plain_wallet"; has many addresses);

#[derive(Debug)]
pub struct PlainUtxo {
    pub prev_hash: String,
    pub prev_index: u64,
    pub address: Address,
    pub amount: u64,
}

impl Wallet for PlainWallet {
    type Utxo = PlainUtxo;

    fn get_utxos(&self, _exec: &Executor) -> Vec<Self::Utxo> {
        vec![]
        /*
        vec![PlainUtxo {
            prev_hash: "abc".to_string(),
            prev_index: 1,
            address: Address{id: "abc".to_string()},
            amount: 100000000,
        }]
        */
    }
}

from_data_wallet!(PlainWallet);

impl ResourceWallet<Address> for PlainWallet {
    fn raw_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    fn merge(self, newer: Self) -> Self {
      let addresses = self.addresses;
      PlainWallet{ addresses, ..newer }
    }

    /*
    fn add_address(&mut self, address: Address) -> Result<bool, String> {
        match self
            .addresses
            .clone()
            .into_iter()
            .find(|in_address| in_address == &address)
        {
            Some(_) => Err(format!("Duplicate address {:?}", address)),
            None => {
                self.addresses.push(address);
                Ok(true)
            }
        }
    }

    fn get_addresses(&self) -> Vec<Address> {
        self.addresses.clone()
    }

    fn remove_address(&mut self, address: Address) -> Result<bool, String> {
        match self
            .addresses
            .clone()
            .into_iter()
            .position(|in_address| in_address == address)
        {
            Some(index) => {
                self.addresses.remove(index);
                Ok(true)
            }
            None => Err(format!("Address {:?} does not exists", address)),
        }
    }
    */
}
