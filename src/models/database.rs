use tiny_ram_db::{ PlainTable };

use models::hd_wallet::HdWallet;
use models::multisig_wallet::MultisigWallet;
use models::plain_wallet::PlainWallet;

pub struct Database {
    pub plain_wallets: PlainTable<PlainWallet>,
    pub hd_wallets: PlainTable<HdWallet>,
    pub multisig_wallets: PlainTable<MultisigWallet>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            plain_wallets: PlainTable::new(),
            hd_wallets: PlainTable::new(),
            multisig_wallets: PlainTable::new()
        }
    }
}