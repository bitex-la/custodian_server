#![feature(ptr_internals)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate bitprim;
extern crate ctrlc;
extern crate libc;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

pub mod server_state;
pub mod wallet;
pub mod handlers;
