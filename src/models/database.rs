use tiny_ram_db::Table;

use models::hd_wallet::{HdWallet, HdWalletIndex};
use models::multisig_wallet::{MultisigWallet, MultisigWalletIndex};
use models::plain_wallet::{PlainWallet, PlainWalletIndex};
use models::plain_address::{ PlainAddress, AddressIndex };
use models::hd_address::{ HdAddress, HdAddressIndex };
use models::multisig_address::{ MultisigAddress, MultisigAddressIndex };

#[derive(Clone)]
pub struct Database {
    pub plain_wallets: Table<PlainWallet, PlainWalletIndex>,
    pub hd_wallets: Table<HdWallet, HdWalletIndex>,
    pub multisig_wallets: Table<MultisigWallet, MultisigWalletIndex>,
    pub plain_addresses: Table<PlainAddress, AddressIndex>,
    pub hd_addresses: Table<HdAddress, HdAddressIndex>,
    pub multisig_addresses: Table<MultisigAddress, MultisigAddressIndex>
}

impl Database {
    pub fn new() -> Self {
        Database {
            plain_wallets: Table::new(),
            hd_wallets: Table::new(),
            multisig_wallets: Table::new(),
            plain_addresses: Table::new(),
            hd_addresses: Table::new(),
            multisig_addresses: Table::new()
        }
    }
}
