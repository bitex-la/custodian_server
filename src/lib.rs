#![feature(ptr_internals)]
#![feature(plugin)]
#![feature(associated_type_defaults)]
#![feature(toowned_clone_into)]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

extern crate bitprim;
extern crate ctrlc;
extern crate libc;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate jsonapi;
extern crate bitcoin;
extern crate tiny_ram_db;
#[macro_use]
extern crate maplit;

extern crate queryst;

#[macro_use]
pub mod data_guards;
pub mod handlers;
pub mod models;
pub mod server_state;
pub mod serializers;
