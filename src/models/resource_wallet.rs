use std::marker::Sized;
use std::fmt::Debug;
use tiny_ram_db::PlainTable;
use models::resource_address::ResourceAddress;
use models::wallet::Wallet;
use models::database::Database;

pub trait ResourceWallet:
    Sized + Clone + Debug + Wallet
{
    type A: ResourceAddress;

    fn default_fields() -> &'static str;

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self>;

}
