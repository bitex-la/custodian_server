use std::fmt::Display;
use std::marker::Sized;
use models::database::Database;
use tiny_ram_db::Table;

pub trait Address: Display + Sized {
    type Index;

    fn addresses_from_database<'a>(database: &'a mut Database) -> &'a mut Table<Self, Self::Index>;
 }