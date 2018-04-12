#![feature(ptr_internals)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate libc;
extern crate rocket;
extern crate ctrlc;
extern crate bitprim;
mod server_state;
mod wallet;
use server_state::ServerState;
use std::fs::File;
use bitprim::PaymentAddress;
use wallet::Wallet;

#[cfg(test)] mod tests;

#[get("/")]
fn hello(state: &ServerState) -> String {
  let chain = state.executor.get_chain();
  let mut wallets = state.wallets_lock();
  wallets.push(Wallet::Legacy{
    id: "hello".to_string(),
    version: "hello".to_string(),
    addresses: vec!["hello".to_string()]
  });

  let addr = PaymentAddress::from_str("mqETuaBY9Tiq1asdsehEyQgCHe34SrXQs9");
  let hist = chain.get_history(addr, 1000, 1).unwrap();

  format!("Block: {:?}. Points: {:?}. Wallets: {:?}",
          chain.get_last_height().expect("height"),
          hist.count(),
          *wallets)
}

#[post("/plain_wallets")]
fn add_plain_wallet(state: &ServerState) {
}

#[post("/hd_wallets")]
fn add_plain_wallet(state: &ServerState) {
}

#[get("/stop")]
fn stop(state: &ServerState) -> String {
  state.graceful_stop();
  format!("Stopping soon.")
}

fn main() {
  let f = File::create("/dev/null").unwrap();
  let state = ServerState::new("./tests/btc-testnet.cfg", &f, &f);
  
  ctrlc::set_handler(move || {
    println!("Do not signal. Stop by visiting /stop");
  }).expect("Error setting Ctrl-C handler");
	
  rocket::ignite()
    .manage(state)
    .mount("/", routes![hello, stop]).launch();

}
