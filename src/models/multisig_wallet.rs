use std::io::Read;
use std::str;
use std::str::FromStr;

use bitcoin::util::bip32::ExtendedPubKey;
use bitprim::explorer::Received;
use jsonapi::model::*;
use tiny_ram_db::PlainTable;

use models::database::Database;
use models::multisig_address::MultisigAddress;
use models::resource_transaction::JsonApiModelTransaction;
use models::resource_wallet::ResourceWallet;
use models::transaction::Transaction;
use models::wallet::Wallet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisigWallet {
    pub label: String,
    pub version: String,
    pub xpubs: Vec<String>,
    pub signers: u64,
}

from_data!(ResourceWallet<MultisigWallet>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigUtxo {
    pub address: MultisigAddress,
    pub script_type: String,
    pub multisig: MultisigDefinition,
    pub transaction: Transaction,
}

impl JsonApiModelTransaction for MultisigUtxo {
    fn jsonapi_type() -> &'static str {
        "multisig_utxo"
    }
}

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
    type RA = MultisigAddress;

    fn construct_utxo(&self, received: Received, address: &MultisigAddress) -> Self::Utxo {
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
            address: address.clone(),
            script_type: "SPENDMULTISIG".to_string(),
            multisig: MultisigDefinition {
                signatures: self.xpubs.iter().map(|_s| String::new()).collect(),
                m: self.xpubs.len(),
                pubkeys,
            },
            transaction: Transaction::new(received, address.to_string()),
        }
    }

    fn jsonapi_type() -> &'static str {
        "multisig_wallet"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self> {
        &mut database.multisig_wallets
    }
}
