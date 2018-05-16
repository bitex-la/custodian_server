#![feature(ptr_internals)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate bitprim;
extern crate ctrlc;
extern crate libc;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate jsonapi;

#[macro_use] mod data_guards;
mod server_state;
mod handlers;
mod models;
use server_state::ServerState;
use std::fs::File;
use handlers::plain_wallets;
use handlers::hd_wallets;
use handlers::multisig_wallets;
use handlers::addresses;
use handlers::hd_addresses;

#[cfg(test)]
mod tests;

#[get("/stop")]
fn stop(state: &ServerState) -> String {
    state.graceful_stop();
    format!("Stopping soon.")
}

fn main() {
    let f = File::create("/dev/null").expect("/dev/null not available");

    let state : ServerState = ServerState::new("./tests/btc-testnet.cfg", &f, &f).expect("Error creating State");

    ctrlc::set_handler(move || {
        println!("Do not signal. Stop by visiting /stop");
    }).expect("Error setting Ctrl-C handler");

    rocket::ignite()
        .manage(state)
        .mount("/", routes![plain_wallets::index, plain_wallets::show, plain_wallets::create, plain_wallets::update, plain_wallets::destroy,
                            hd_wallets::index, hd_wallets::show, hd_wallets::create, hd_wallets::update, hd_wallets::destroy,
                            multisig_wallets::index, multisig_wallets::show, multisig_wallets::create, multisig_wallets::update, multisig_wallets::destroy,
                            addresses::create, addresses::destroy,
                            hd_addresses::create, hd_addresses::destroy,
                            stop])
        .launch();
}
