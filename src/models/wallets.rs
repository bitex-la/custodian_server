use std::clone::Clone;

use jsonapi::model::*;
use models::hd_wallet::HdWallet;
use models::multisig_wallet::MultisigWallet;
use models::plain_wallet::PlainWallet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallets {
    pub id: Option<String>,
    pub plains: Vec<PlainWallet>,
    pub hds: Vec<HdWallet>,
    pub multisigs: Vec<MultisigWallet>,
}

jsonapi_model!(Wallets; "wallets"; has many plains, hds, multisigs);
