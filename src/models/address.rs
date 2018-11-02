use std::clone::Clone;
use std::collections::hash_set::HashSet;
use jsonapi::model::Query;
use models::database::Database;
use models::wallet::Wallet;
use tiny_ram_db;
use tiny_ram_db::{ Table, Record };
use jsonapi::model::JsonApiModel;
use models::hd_wallet::HdWallet;
use models::hd_address::HdAddress;

/*  Addresses belong to a wallet, can be indexed, and have an associated
 *  public Bitcoin address.
 */
pub trait Address: Clone {
    type Index;
    type Wallet: Wallet;

    fn public(&self) -> String;

    fn addresses_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index>;

    fn filter_by_wallet<'a>(wallet_id: usize, database: &'a mut Database)
      -> Result<HashSet<Record<Self>>, tiny_ram_db::errors::Error>;

    fn get_record_wallet(&self) -> Record<Self::Wallet>;
}
