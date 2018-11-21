use std::str;
use std::str::FromStr;
use serde_json;
use tiny_ram_db;
use tiny_ram_db::hashbrown;

use bitcoin::util::bip32::ExtendedPubKey;
use bitprim::explorer::Received;
use jsonapi::model::*;
use tiny_ram_db::{Table, Record, Index, Indexer};

use models::database::Database;
use models::multisig_address::MultisigAddress;
use models::transaction::Transaction;
use models::wallet::Wallet;
use models::address::Address;
use serializers::{FromJsonApi, ToJsonApi};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisigWallet {
    pub label: String,
    pub version: String,
    pub xpubs: Vec<String>,
    pub signers: u64,
    #[serde(skip_deserializing)]
    pub balance: Option<u64>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigUtxo {
    pub address: Record<MultisigAddress>,
    pub script_type: String,
    pub multisig: MultisigDefinition,
    pub transaction: Transaction,
}

#[derive(Default)]
pub struct MultisigWalletIndex {
    pub by_label: Index<String, MultisigWallet>,
}

impl MultisigWalletIndex {
    fn remove(&mut self, label: String) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_label.data.remove(&label);
        Ok(true)
    }
}

impl Indexer for MultisigWalletIndex {
    type Item = MultisigWallet;
    fn index(&mut self, item: &Record<MultisigWallet>) -> Result<bool, tiny_ram_db::errors::Error> {
        self.by_label
            .insert(item.data.label.clone(), item.clone())?;
        Ok(true)
    }
}

impl ToJsonApi for MultisigUtxo {
    const TYPE : &'static str = "multisig_utxos";

    fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
        hashmap!{
            "script_type".to_string() => serde_json::to_value(&self.script_type).unwrap(),
            "multisig".to_string() => serde_json::to_value(&self.multisig).unwrap(),
            "transaction".to_string() => serde_json::to_value(&self.transaction).unwrap()
        }
    }

    fn relationships(&self, _fields: &QueryFields) -> Option<Relationships> {
        Some(hashmap!{
            "address".to_string() => Self::has_one("multisig_addresses", self.address.id),
        })
    }

    fn included(&self, _fields: &Vec<String>) -> Option<Resources> {
        Some(vec![self.address.data.to_jsonapi_resource(self.address.id).0])
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
    type Index = MultisigWalletIndex;
    type Utxo = MultisigUtxo;
    type RA = MultisigAddress;

    fn construct_utxo(&self, received: Received, address: Record<MultisigAddress>) -> Self::Utxo {
        let pubkeys = self
            .xpubs
            .iter()
            .map(|xpub| {
                let (chain_code, pub_key) =
                    if let Ok(extended_pub_key) = ExtendedPubKey::from_str(xpub) {
                        (
                            self.slice_to_hex(&extended_pub_key.chain_code.to_bytes()),
                            self.slice_to_hex(&extended_pub_key.public_key.serialize()),
                        )
                    } else {
                        (String::new(), String::new())
                    };

                PubkeyDefinition {
                    address_n: address.data.path.clone(),
                    node: NodeDefinition {
                        chain_code,
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
            transaction: Transaction::new(received, address.data.public()),
        }
    }

    fn jsonapi_type() -> &'static str {
        "multisig_wallet"
    }

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index> {
        &mut database.multisig_wallets
    }

    fn update_attributes<'a>(&self, version: String, balance: u64) -> Self{
        MultisigWallet {
            version,
            xpubs: self.xpubs.clone(),
            label: self.label.clone(),
            signers: self.signers.clone(),
            balance: Some(balance)
        }
    }

    fn by_label<'a>(label: String, database: &'a mut Database)
        -> Result<hashbrown::HashSet<Record<Self>>, tiny_ram_db::errors::Error> {
            database
                .multisig_wallets
                .indexes
                .read()?
                .by_label
                .get(&label, |items| items.clone())
        }

    fn get_label(&self) -> String {
        self.label.clone()
    }

    fn remove_from_indexes<'a>(table: &'a Table<Self, Self::Index>, id: String) -> Result<bool, tiny_ram_db::errors::Error> {
        let mut indexes = table.indexes.write().expect("Error getting write access to indexes");
        indexes.remove(id)?;
        Ok(true)
    }
}

impl ToJsonApi for MultisigWallet {
    const TYPE : &'static str = "multisig_wallets";

    fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
        hashmap!{
            "version".to_string() => serde_json::to_value(&self.version).unwrap(),
            "xpubs".to_string() => serde_json::to_value(&self.xpubs).unwrap(),
            "label".to_string() => serde_json::to_value(&self.label).unwrap(),
            "signers".to_string() => serde_json::to_value(&self.signers).unwrap(),
            "balance".to_string() => serde_json::to_value(&self.balance).unwrap(),
        }
    }
}

impl FromJsonApi for MultisigWallet {
    const TYPE : &'static str = "multisig_wallets";

    fn from_json_api_resource(resource: Resource, _db: Database) -> Result<Self, String> {
        Ok(MultisigWallet{
            version: Self::attribute(&resource, "version")?,
            xpubs: Self::attribute(&resource, "xpubs")?,
            label: Self::attribute(&resource, "label")?,
            signers: Self::attribute(&resource, "signers")?,
            balance: None
        })
    }
}

