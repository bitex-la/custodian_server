use std::fmt::Display;
use std::marker::Sized;
use std::clone::Clone;
use std::collections::hash_set::HashSet;
use models::database::Database;
use tiny_ram_db;
use tiny_ram_db::{ Table, Record };

pub trait Address: Display + Sized + Clone {
    type Index;

    fn addresses_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index>;
    fn filter_by_wallet<'a>(wallet_id: usize, database: &'a mut Database) -> Result<HashSet<Record<Self>>, tiny_ram_db::errors::Error>;
 }