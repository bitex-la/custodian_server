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

impl Wallets {
    /*
    pub fn add_address<W: ResourceWallet<A>, A: ResourceAddress>(
        state_wallets: &mut Vec<W>,
        id: u64,
        address: A,
    ) -> Result<bool, String> {
    }

    pub fn destroy_address<W: ResourceWallet<A> + Clone, A: ResourceAddress + Debug>(
        state_wallets: &mut Vec<W>,
        id: u64,
        address: A,
    ) -> Result<bool, String> {
        let index = state_wallets.iter().position(|wallet| wallet.id() == id);
        match index {
            Some(value) => state_wallets[value].remove_address(address),
            None => Err(format!("{:?}", id)),
        }
    }
    */
}
