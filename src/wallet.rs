pub trait Wallet {
  type Utxo;

  pub fn get_utxos(&self, exec: &Executor) -> Vec<Self::Utxo>;
}

#[derive(Debug)]
struct PlainWallet {
  id: String,
  version: String,
  addresses: Vec<String>
}

#[derive(Debug)]
struct PlainUtxo {
  prev_hash: String,
  prev_index: u64,
  address: String,
  amount: u64
}

impl Wallet for PlainWallet {
  type Utxo = PlainUtxo;

  pub fn get_utxos(&self, exec: &Executor) -> Vec<Self::Utxo> {
    vec![PlainUtxo{
      prev_hash: "abc".to_string(),
      prev_index: 1,
      address: "abc".to_string(),
      amount: 100000000
    }]
  }
}

#[derive(Debug)]
pub struct HdWallet {
  pub id: String,
  pub version: String,
  pub addresses: Vec<HdAddress>,
  pub xpub: String
}

pub struct HdUtxo {
  pub prev_hash: String,
  pub prev_index: u64,
  pub address: HdAddress,
  pub amount: u64
}

impl Wallet for HdWallet {
  type Utxo = HdUtxo;

  pub fn get_utxos(&self, exec: &Executor) -> Vec<Self::Utxo> {
    vec![HdUtxo{
      prev_hash: "abc".to_string(),
      prev_index: 1,
      address: HdAddress{address: "abc".to_string(), path: vec![0,1,0]},
      amount: 100000000
    }]
  }
}

#[derive(Debug)]
pub struct Multisig {
  pub id: String,
  pub version: String,
  pub addresses: Vec<HdAddress>,
  pub xpubs: Vec<String>
  pub signers: u64
}

pub struct MultisigUtxo {
  pub prev_hash: String,
  pub prev_index: u64,
  pub address: HdAddress,
  pub amount: u64
  pub script_type: String,
  pub multisig: MultisigDefinition
}

pub struct MultisigDefinition {
  pub signatures: Vec<String>,
  pub m: u64,
  pub pubkeys: Vec<PubkeyDefinition>
}

pub struct PubkeyDefinition {
  pub address_n: Vec<u64>,1,1],
  pub node: NodeDefiniton
}

pub struct NodeDefiniton {
  pub chain_code: String,
  pub depth: u64, 
  pub child_num: u64, 
  pub fingerprint: u64,
  pub public_key: String
}

impl Wallet for HdWallet {
  type Utxo = HdUtxo;
  pub fn get_utxos(&self, exec: &Executor) -> Vec<Self::Utxo> {
    let pubkeys = self.xpubs.iter().map(|xpub|{
      PubkeyDefinition {
        address_n: vec![0,0,1],
        node: NodeDefinition {
          chain_code: "Hello".to_string(),
          depth: 0,
          child_num: 0,
          fingerprint: 0,
          public_key: xpub
        }
      }
    }).collect();

    vec![MultisigUtxo{
      prev_hash: "abc".to_string(),
      prev_index: 1,
      address: HdAddress{address: "abc".to_string(), path: vec![0,1,0]},
      amount: 100000000,
      script_type: "SPENDMULTISIG",
      multisig: MultisigDefinition {
        signatures: vec!["","",""],
        m: self.xpubs.len(),
        pubkeys: pubkeys
      }
    }]
  }

#[derive(Debug)]
pub struct HdAddress {
  pub address: String,
  pub path: Vec<u64>
}
