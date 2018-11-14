use std::clone::Clone;
use tiny_ram_db::hashbrown;
use models::database::Database;
use models::wallet::Wallet;
use tiny_ram_db;
use tiny_ram_db::{ Table, Record };

/*  Addresses belong to a wallet, can be indexed, and have an associated
 *  public Bitcoin address.
 */
pub trait Address: Clone {
    type Index;
    type Wallet: Wallet;

    fn public(&self) -> String;

    fn by_wallet<'a>(wallet_id: usize, database: &'a mut Database)
      -> Result<hashbrown::HashSet<Record<Self>>, tiny_ram_db::errors::Error>;

    fn get_record_wallet(&self) -> Record<Self::Wallet>;

    fn table<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index>;

    fn remove_from_indexes<'a>(table: &'a Table<Self, Self::Index>, id: &'a usize) -> Result<bool, tiny_ram_db::errors::Error>;
}
