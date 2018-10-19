use tiny_ram_db::HasMany;
use tiny_ram_db::Record;
use tiny_ram_db::Table;

use models::hd_wallet::HdWallet;
use models::multisig_wallet::MultisigWallet;
use models::plain_wallet::PlainWallet;

pub struct Database {
    pub plain_wallets: Table<PlainWallet>,
    pub hd_wallets: Table<HdWallet>,
    pub multisig_wallets: Table<MultisigWallet>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            plain_wallets: Table::new(),
            hd_wallets: Table::new(),
            multisig_wallets: Table::new()
        }
    }
}