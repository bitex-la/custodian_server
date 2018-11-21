#![feature(ptr_internals)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(toowned_clone_into)]

extern crate bitprim;
extern crate ctrlc;
extern crate libc;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
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
mod data_guards;
mod handlers;
mod models;
mod serializers;
mod server_state;

use handlers::{addresses, blocks, transactions, wallets};
use server_state::ServerState;
use std::env;
use std::io;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins};

#[cfg(test)]
mod tests;

#[get("/stop")]
fn stop(state: &ServerState) -> String {
    state.graceful_stop();
    "Stopping soon.".to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (allowed_origins, _) = AllowedOrigins::some(&["http://localhost", "http://34.198.61.110"]);
    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect(),
        allow_credentials: true,
        ..Default::default()
    };
    let conf_path = &args
        .get(1)
        .expect("You need to provide a path for a config file.");

    let state: ServerState =
        ServerState::new(conf_path, &io::stdout(), &io::stderr()).expect("Error creating State");

    ctrlc::set_handler(move || {
        println!("Do not signal. Stop by visiting /stop");
    }).expect("Error setting Ctrl-C handler");

    let routes = 
        routes![
                transactions::base::broadcast,
                wallets::plain::index,
                wallets::plain::show,
                wallets::plain::create,
                wallets::plain::update,
                wallets::plain::destroy,
                wallets::plain::get_utxos,
                wallets::plain::get_incoming,
                wallets::hd::index,
                wallets::hd::show,
                wallets::hd::create,
                wallets::hd::update,
                wallets::hd::destroy,
                wallets::hd::get_utxos,
                wallets::hd::get_incoming,
                wallets::multisig::index,
                wallets::multisig::show,
                wallets::multisig::create,
                wallets::multisig::update,
                wallets::multisig::destroy,
                wallets::multisig::get_utxos,
                wallets::multisig::get_incoming,
                addresses::plain::index,
                addresses::plain::create,
                addresses::plain::show,
                addresses::plain::destroy,
                addresses::plain::balance,
                addresses::plain::get_utxos,
                addresses::hd::index,
                addresses::hd::create,
                addresses::hd::show,
                addresses::hd::destroy,
                addresses::hd::balance,
                addresses::hd::get_utxos,
                addresses::multisig::index,
                addresses::multisig::create,
                addresses::multisig::show,
                addresses::multisig::destroy,
                addresses::multisig::balance,
                addresses::multisig::get_utxos,
                blocks::base::last,
                stop
            ];

    rocket::ignite()
        .manage(state)
        .mount("/", routes)
        .attach(options)
        .launch();
}
