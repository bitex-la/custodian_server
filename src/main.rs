#![feature(ptr_internals)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
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

extern crate bitcoin;

#[macro_use]
mod data_guards;
mod handlers;
mod models;
mod server_state;
use handlers::{addresses, wallets};
use server_state::ServerState;
use std::fs::File;

#[cfg(test)]
mod tests;

#[get("/stop")]
fn stop(state: &ServerState) -> String {
    state.graceful_stop();
    "Stopping soon.".to_string()
}

fn main() {
    let f = File::create("/dev/null").expect("/dev/null not available");

    let state: ServerState =
        ServerState::new("./tests/btc-testnet.cfg", &f, &f).expect("Error creating State");

    ctrlc::set_handler(move || {
        println!("Do not signal. Stop by visiting /stop");
    }).expect("Error setting Ctrl-C handler");

    rocket::ignite()
        .manage(state)
        .mount(
            "/",
            routes![
                wallets::plain::index,
                wallets::plain::show,
                wallets::plain::create,
                wallets::plain::update,
                wallets::plain::destroy,
                wallets::plain::get_utxos,
                wallets::hd::index,
                wallets::hd::show,
                wallets::hd::create,
                wallets::hd::update,
                wallets::hd::destroy,
                wallets::hd::get_utxos,
                wallets::multisig::index,
                wallets::multisig::show,
                wallets::multisig::create,
                wallets::multisig::update,
                wallets::multisig::destroy,
                wallets::multisig::get_utxos,
                addresses::plain::index,
                addresses::plain::create,
                addresses::plain::destroy,
                addresses::hd::index,
                addresses::hd::create,
                addresses::hd::destroy,
                addresses::multisig::index,
                addresses::multisig::create,
                addresses::multisig::destroy,
                stop
            ],
        )
        .launch();
}
