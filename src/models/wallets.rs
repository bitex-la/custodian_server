use std::mem;
use std::iter::Iterator;

use jsonapi::model::*;
use models::plain_wallet::PlainWallet;
use models::plain_wallet::Address;
use models::hd_wallet::HdWallet;
use models::multisig_wallet::MultisigWallet;
use models::resource_wallet::ResourceWallet;
use models::hd_wallet::HdAddress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallets {
    pub id: String,
    pub plains: Vec<PlainWallet>,
    pub hds: Vec<HdWallet>,
    pub multisigs: Vec<MultisigWallet>
}

jsonapi_model!(Wallets; "wallets"; has many plains, hds, multisigs);

impl Wallets {
    pub fn show_wallet<W : ResourceWallet>(state_wallets: &Vec<W>, id: i32) -> Result<&W, String> {
        let wallet = &state_wallets.iter().find(|&wallet| wallet.id() == id);
        match wallet {
            Some(value) => Ok(*value),
            None        => Err(format!("{:?}", id))
        }
    }

    pub fn update_wallet<W : ResourceWallet>(state_wallets: &mut Vec<W>, id: i32, field_wallet: W) -> Result<bool, String> {
        let index = state_wallets.iter().position(|wallet| wallet.id() == id);
        match index {
            Some(value) => { mem::replace(&mut state_wallets[value], field_wallet); Ok(true) },
            None        => Err(format!("{:?}", id))
        }
    }

    pub fn destroy_wallet<W : ResourceWallet>(state_wallets: &mut Vec<W>, id: i32) -> Result<bool, String> {
        let index = &state_wallets.iter().position(|ref wallet| wallet.id() == id);
        match index {
            Some(index) => { &state_wallets.remove(*index); Ok(true) },
            None        => Err(format!("{:?}", id))
        }
    }

    pub fn add_address(state_wallets: &mut Vec<PlainWallet>, id: i32, address: Address) -> Result<bool, String> {
        let index = state_wallets.iter().position(|wallet| wallet.id() == id);
        match index {
            Some(value) => { state_wallets[value].addresses.push(address); Ok(true) },
            None        => Err(format!("{:?}", id))
        }
    }

    pub fn destroy_address(state_wallets: &mut Vec<PlainWallet>, id: i32, address: Address) -> Result<bool, String> {
        let index = state_wallets.iter().position(|wallet| wallet.id() == id);
        match index {
            Some(value) => {
                let addresses = state_wallets[value].clone().addresses; 
                let address_index = addresses.iter().position(|orig_address| orig_address == &address);
                match address_index {
                    Some(value_address) => { state_wallets[value].addresses.remove(value_address); Ok(true) },
                    None                => Err(format!("{:?}", address))
                }
            },
            None        => Err(format!("{:?}", id))
        }
    }

    pub fn add_hd_address<W : ResourceWallet>(state_wallets: &mut Vec<W>, id: i32, hd_address: HdAddress) -> Result<bool, String> {
        let index = state_wallets.iter().position(|wallet| wallet.id() == id);
        match index {
            Some(value) => { state_wallets[value].add_address(hd_address); Ok(true) },
            None        => Err(format!("{:?}", id))
        }
    }
}
