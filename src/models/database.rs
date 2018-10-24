use tiny_ram_db::{ PlainTable, Table };

use models::hd_wallet::HdWallet;
use models::multisig_wallet::MultisigWallet;
use models::plain_wallet::PlainWallet;
use models::address::{ Address, AddressIndex };

pub struct Database {
    pub plain_wallets: PlainTable<PlainWallet>,
    pub hd_wallets: PlainTable<HdWallet>,
    pub multisig_wallets: PlainTable<MultisigWallet>,
    pub addresses: Table<Address, AddressIndex>
}

impl Database {
    pub fn new() -> Self {
        Database {
            plain_wallets: PlainTable::new(),
            hd_wallets: PlainTable::new(),
            multisig_wallets: PlainTable::new(),
            addresses: Table::new()
        }
    }
}