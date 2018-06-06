use std::io::Read;
use std::str;
use std::str::FromStr;

use bitcoin::util::bip32::ExtendedPubKey;
use bitprim::explorer::Received;
use jsonapi::model::*;

pub use models::hd_wallet::HdAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::wallets::Wallets;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigUtxo {
    pub id: Option<String>,
    pub prev_hash: String,
    pub prev_index: u32,
    pub address: HdAddress,
    pub amount: u64,
    pub script_type: String,
    pub multisig: MultisigDefinition,
}
jsonapi_model!(MultisigUtxo; "multi_utxo");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigDefinition {
    pub signatures: Vec<String>,
    pub m: usize,
    pub pubkeys: Vec<PubkeyDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubkeyDefinition {
    pub address_n: Vec<u64>,
    pub node: NodeDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDefinition {
    pub chain_code: String,
    pub depth: u64,
    pub child_num: u64,
    pub fingerprint: u64,
    pub public_key: String,
}

impl MultisigWallet {
    fn slice_to_hex(&self, arr: &[u8]) -> String {
        arr.iter().map(|c| format!("{:02x}", c)).collect()
    }
}

impl Wallet for MultisigWallet {
    type Utxo = MultisigUtxo;
    type RA = HdAddress;

    fn construct_utxo(&self, received: Received, address: &HdAddress) -> Self::Utxo {
        let pubkeys = self
            .xpubs
            .iter()
            .map(|xpub| {
                let (chain_code, pub_key) =
                    if let Ok(extended_pub_key) = ExtendedPubKey::from_str(xpub) {
                        (
                            self.slice_to_hex(&extended_pub_key.chain_code.data()),
                            self.slice_to_hex(&extended_pub_key.public_key.serialize()),
                        )
                    } else {
                        (String::new(), String::new())
                    };

                PubkeyDefinition {
                    address_n: address.path.clone(),
                    node: NodeDefinition {
                        chain_code: chain_code,
                        depth: 0,
                        child_num: 0,
                        fingerprint: 0,
                        public_key: pub_key,
                    },
                }
            })
            .collect();
        MultisigUtxo {
            id: Some(format!(
                "{}-{}",
                received.transaction_hash, received.position
            )),
            prev_hash: received.transaction_hash,
            prev_index: received.position,
            address: address.clone(),
            amount: received.satoshis,
            script_type: "SPENDMULTISIG".to_string(),
            multisig: MultisigDefinition {
                signatures: self.xpubs.iter().map(|_s| String::new()).collect(),
                m: self.xpubs.len(),
                pubkeys,
            },
        }
    }

    fn get_addresses<'a>(&'a self) -> &'a Vec<HdAddress> {
        &self.addresses
    }
}

from_data!(MultisigWallet);

impl ResourceWallet for MultisigWallet {
    type A = HdAddress;

    fn raw_id(&self) -> Option<u64> {
        self.id
    }

    fn set_id(self, new_id: u64) -> Self {
        MultisigWallet {
            id: Some(new_id),
            ..self
        }
    }

    fn merge(self, newer: Self) -> Self {
        let addresses = self.addresses;
        MultisigWallet {
            addresses,
            id: self.id,
            ..newer
        }
    }

    fn add_address(&mut self, address: Self::A) {
        self.addresses.push(address);
    }

    fn get_addresses<'a>(&'a mut self) -> &'a mut Vec<Self::A> {
        self.addresses.as_mut()
    }

    fn default_fields() -> &'static str {
        "version,xpubs,signers"
    }

    fn address_fields() -> &'static str {
        "address, path"
    }

    fn collection_from_wallets<'a>(wallets: &'a mut Wallets) -> &'a mut Vec<Self> {
        wallets.multisigs.as_mut()
    }

    fn remove_address(&mut self, index: usize) {
        self.addresses.remove(index);
    }

    fn find_address_position(&self, address: &Self::A) -> Option<usize> {
        self.addresses
            .clone()
            .into_iter()
            .position(|in_address| in_address.id == address.id)
    }
}
