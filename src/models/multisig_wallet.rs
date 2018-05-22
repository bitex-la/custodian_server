use std::io::Read;

use bitprim::executor::Executor;
use jsonapi::model::*;

use models::hd_wallet::HdAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigWallet {
    pub id: Option<u64>,
    pub version: String,
    #[serde(default)]
    pub addresses: Vec<HdAddress>,
    pub xpubs: Vec<String>,
    pub signers: u64,
}

jsonapi_model!(MultisigWallet; "multisig_wallet");

pub struct MultisigUtxo {
    pub prev_hash: String,
    pub prev_index: u64,
    pub address: HdAddress,
    pub amount: u64,
    pub script_type: String,
    pub multisig: MultisigDefinition,
}

pub struct MultisigDefinition {
    pub signatures: Vec<String>,
    pub m: usize,
    pub pubkeys: Vec<PubkeyDefinition>,
}

pub struct PubkeyDefinition {
    pub address_n: Vec<u64>,
    pub node: NodeDefinition,
}

pub struct NodeDefinition {
    pub chain_code: String,
    pub depth: u64,
    pub child_num: u64,
    pub fingerprint: u64,
    pub public_key: String,
}

impl Wallet for MultisigWallet {
    type Utxo = MultisigUtxo;

    fn get_utxos(&self, _exec: &Executor) -> Vec<Self::Utxo> {
        vec![]
        /*let pubkeys = self.xpubs
            .iter()
            .map(|xpub| PubkeyDefinition {
                address_n: vec![0, 0, 1],
                node: NodeDefinition {
                    chain_code: "Hello".to_string(),
                    depth: 0,
                    child_num: 0,
                    fingerprint: 0,
                    public_key: xpub.to_string(),
                },
            })
            .collect();
        vec![MultisigUtxo {
            prev_hash: "abc".to_string(),
            prev_index: 1,
            address: HdAddress {
                id: "1".to_string(),
                address: "abc".to_string(),
                path: vec![0, 1, 0],
            },
            amount: 100000000,
            script_type: "SPENDMULTISIG".to_string(),
            multisig: MultisigDefinition {
                signatures: vec![String::new(), String::new(), String::new()],
                m: self.xpubs.len(),
                pubkeys
            },
        }]
        */
    }
}

from_data_wallet!(MultisigWallet);

impl ResourceWallet<HdAddress> for MultisigWallet {
    fn raw_id(&self) -> Option<u64> {
        self.id
    }

    fn set_id(self, new_id: u64) -> Self {
        MultisigWallet { id: Some(new_id), ..self }
    }

    fn merge(self, newer: Self) -> Self {
      let addresses = self.addresses;
      MultisigWallet{ addresses, ..newer }
    }

    fn add_address(&mut self, address: HdAddress)-> Result<bool, String> {
        match self.addresses.clone().into_iter().find(|in_address| in_address.id == address.id) {
            Some(_) => Err(format!("Duplicate address {:?}", address)),
            None => {
                self.addresses.push(address);
                Ok(true)
            }
        }
    }

    /*
    fn get_addresses(&self) -> Vec<HdAddress> {
        self.addresses.clone()
    }

    fn remove_address(&mut self, address: HdAddress) -> Result<bool, String> {
        match self
            .addresses
            .clone()
            .into_iter()
            .position(|in_address| in_address.id == address.id)
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
