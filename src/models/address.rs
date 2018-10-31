use std::fmt::Display;
use std::marker::Sized;
use std::clone::Clone;
use std::collections::hash_set::HashSet;
use jsonapi::model::Query;
use models::database::Database;
use models::wallet::Wallet;
use tiny_ram_db;
use tiny_ram_db::{ Table, Record };

pub trait Address: Display + Sized + Clone {
    type Index;
    type Wallet: Wallet;

    fn addresses_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index>;
    fn filter_by_wallet<'a>(wallet_id: usize, database: &'a mut Database) -> Result<HashSet<Record<Self>>, tiny_ram_db::errors::Error>;

    fn jsonapi_type() -> &'static str;

    fn default_query() -> Query {
        Query::from_params(&format!(
            "include=[]&fields[{}]={}",
            Self::jsonapi_type(),
            "address"
        ))
    }

    fn get_record_wallet(&self) -> Record<Self::Wallet>;
 }