#![feature(ptr_internals)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate bitprim;
extern crate ctrlc;
extern crate libc;
extern crate rocket;
mod server_state;
mod wallet;
use server_state::ServerState;
use std::fs::File;
use std::ops::DerefMut;
use bitprim::PaymentAddress;
use bitprim::explorer::OpaqueCollection;
use wallet::PlainWallet;

#[cfg(test)]
mod tests;

#[get("/")]
fn hello_plain_wallet(state: &ServerState<PlainWallet>) -> String {
    let chain = state.executor.get_chain();
    let mut wallets = state.wallets_lock();
    wallets.deref_mut().push(PlainWallet {
        id: "hello".to_string(),
        version: "hello".to_string(),
        addresses: vec!["hello".to_string()],
    });

    let addr = PaymentAddress::from_str("mqETuaBY9Tiq1asdsehEyQgCHe34SrXQs9");
    let hist = chain.get_history(addr, 1000, 1).unwrap();

    format!(
        "Block: {:?}. Points: {:?}. Wallets: {:?}",
        chain.get_last_height().expect("height"),
        hist.len(),
        *wallets
    )
}

#[get("/stop")]
fn stop_plain_wallet(state: &ServerState<PlainWallet>) -> String {
    state.graceful_stop();
    format!("Stopping soon.")
}

fn main() {
    let f = File::create("/dev/null").expect("/dev/null not available");

    let state : ServerState<PlainWallet> = ServerState::new("./tests/btc-testnet.cfg", &f, &f).expect("Error creating State");

    ctrlc::set_handler(move || {
        println!("Do not signal. Stop by visiting /stop");
    }).expect("Error setting Ctrl-C handler");

    rocket::ignite()
        .manage(state)
        .mount("/", routes![hello_plain_wallet, stop_plain_wallet])
        .launch();
}
