#![feature(ptr_internals)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(associated_type_defaults)]
#![feature(custom_derive)]

extern crate bitprim;
extern crate ctrlc;
extern crate libc;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate jsonapi;

#[macro_use]
pub mod data_guards;
pub mod handlers;
pub mod models;
pub mod server_state;
