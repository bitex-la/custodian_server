use std::mem;
use std::iter::Iterator;
use std::clone::Clone;
use std::fmt::Debug;
use std::cmp::PartialEq;

use jsonapi::model::*;
use models::plain_wallet::PlainWallet;
use models::hd_wallet::HdWallet;
use models::multisig_wallet::MultisigWallet;
use models::resource_wallet::ResourceWallet;
use models::resource_address::ResourceAddress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallets {
    pub id: String,
    pub plains: Vec<PlainWallet>,
    pub hds: Vec<HdWallet>,
    pub multisigs: Vec<MultisigWallet>
}

jsonapi_model!(Wallets; "wallets"; has many plains, hds, multisigs);

impl Wallets {
    pub fn show_wallet<W : ResourceWallet<A>, A: ResourceAddress>(state_wallets: &Vec<W>, id: i32) -> Result<&W, String> {
        let wallet = &state_wallets.iter().find(|&wallet| wallet.id() == id);
        match wallet {
            Some(value) => Ok(*value),
            None        => Err(format!("{:?}", id))
        }
    }

    pub fn update_wallet<W : ResourceWallet<A>, A: ResourceAddress>(state_wallets: &mut Vec<W>, id: i32, field_wallet: W) -> Result<bool, String> {
        let index = state_wallets.iter().position(|wallet| wallet.id() == id);
        match index {
            Some(value) => { mem::replace(&mut state_wallets[value], field_wallet); Ok(true) },
            None        => Err(format!("{:?}", id))
        }
    }

    pub fn destroy_wallet<W : ResourceWallet<A>, A: ResourceAddress>(state_wallets: &mut Vec<W>, id: i32) -> Result<bool, String> {
        let index = &state_wallets.iter().position(|ref wallet| wallet.id() == id);
        match index {
            Some(index) => { &state_wallets.remove(*index); Ok(true) },
            None        => Err(format!("{:?}", id))
        }
    }

    pub fn add_address<W: ResourceWallet<A>, A: ResourceAddress>(state_wallets: &mut Vec<W>, id: i32, address: A) -> Result<bool, String> {
        let index = state_wallets.iter().position(|wallet| wallet.id() == id);
        match index {
            Some(value) => { state_wallets[value].add_address(address); Ok(true) },
            None        => Err(format!("{:?}", id))
        }
    }

    pub fn destroy_address<W: ResourceWallet<A> + Clone, A: ResourceAddress + Debug + PartialEq>(state_wallets: &mut Vec<W>, id: i32, address: A) -> Result<bool, String> {
        let index = state_wallets.iter().position(|wallet| wallet.id() == id);
        match index {
            Some(value) => {
                let addresses = state_wallets[value].get_addresses(); 
                let address_index = addresses.iter().position(|orig_address| orig_address == &address);
                match address_index {
                    Some(value_address) => { state_wallets[value].remove_address(value_address); Ok(true) },
                    None                => Err(format!("{:?}", address))
                }
            },
            None        => Err(format!("{:?}", id))
        }
    }
}
