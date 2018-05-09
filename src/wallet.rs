use bitprim::executor::Executor;

pub trait Wallet {
    type Utxo;

    fn get_utxos(&self, exec: &Executor) -> Vec<Self::Utxo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainWallet {
    pub id: String,
    pub version: String,
    pub addresses: Vec<String>,
}

#[derive(Debug)]
pub struct PlainUtxo {
    pub prev_hash: String,
    pub prev_index: u64,
    pub address: String,
    pub amount: u64,
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

#[derive(Debug, Clone)]
pub struct HdWallet {
    pub id: String,
    pub version: String,
    pub addresses: Vec<HdAddress>,
    pub xpub: String,
}

pub struct HdUtxo {
    pub prev_hash: String,
    pub prev_index: u64,
    pub address: HdAddress,
    pub amount: u64,
}

impl Wallet for HdWallet {
    type Utxo = HdUtxo;

    fn get_utxos(&self, _exec: &Executor) -> Vec<Self::Utxo> {
        vec![
            HdUtxo {
                prev_hash: "abc".to_string(),
                prev_index: 1,
                address: HdAddress {
                    address: "abc".to_string(),
                    path: vec![0, 1, 0],
                },
                amount: 100000000,
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct MultisigWallet {
    pub id: String,
    pub version: String,
    pub addresses: Vec<HdAddress>,
    pub xpubs: Vec<String>,
    pub signers: u64,
}

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
        let pubkeys = self.xpubs
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

        vec![
            MultisigUtxo {
                prev_hash: "abc".to_string(),
                prev_index: 1,
                address: HdAddress {
                    address: "abc".to_string(),
                    path: vec![0, 1, 0],
                },
                amount: 100000000,
                script_type: "SPENDMULTISIG".to_string(),
                multisig: MultisigDefinition {
                    signatures: vec![String::new(), String::new(), String::new()],
                    m: self.xpubs.len(),
                    pubkeys: pubkeys,
                },
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct HdAddress {
    pub address: String,
    pub path: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct Wallets {
    pub plain_wallets: Vec<PlainWallet>,
    pub hd_wallets: Vec<HdWallet>,
    pub multisig_wallets: Vec<MultisigWallet>
}
