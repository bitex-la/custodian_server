use tiny_ram_db::{ PlainTable, Table };

use models::hd_wallet::HdWallet;
use models::multisig_wallet::MultisigWallet;
use models::plain_wallet::PlainWallet;
use models::plain_address::{ PlainAddress, AddressIndex };
use models::hd_address::{ HdAddress, HdAddressIndex };
use models::multisig_address::{ MultisigAddress, MultisigAddressIndex };

pub struct Database {
    pub plain_wallets: PlainTable<PlainWallet>,
    pub hd_wallets: PlainTable<HdWallet>,
    pub multisig_wallets: PlainTable<MultisigWallet>,
    pub addresses: Table<PlainAddress, AddressIndex>,
    pub hd_addresses: Table<HdAddress, HdAddressIndex>,
    pub multisig_addresses: Table<MultisigAddress, MultisigAddressIndex>
}

impl Database {
    pub fn new() -> Self {
        Database {
            plain_wallets: PlainTable::new(),
            hd_wallets: PlainTable::new(),
            multisig_wallets: PlainTable::new(),
            addresses: Table::new(),
            hd_addresses: Table::new(),
            multisig_addresses: Table::new()
        }
    }
}