#![feature(ptr_internals)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate libc;
extern crate rocket;
extern crate ctrlc;
mod bitprim;
mod server_state;
mod wallet;
use rocket::State;
use server_state::{GuardedServerState, ServerState};
use std::sync::atomic::{ATOMIC_BOOL_INIT, AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)] mod tests;

#[get("/")]
fn hello(guard: GuardedServerState) -> String {
	format!("block: {:?} {:?}",
    guard.state.bitprim.get_address_history("mjQx3W3AcPTC73KiknrGNgt5K5YM7cffrx", 200, 0),
    guard.state.bitprim.last_height())
}

#[get("/stop")]
fn stop(guard: GuardedServerState) -> String {
  println!("Ok, I was told to stop.");
  guard.state.graceful_stop();
  format!("")
}

fn main() {
  let mut f = File::create("/dev/null").unwrap();
  let state = ServerState::new("/home/nubis/btc-testnet.cfg", &f, &f);
	/*println!("{:?}", state.bitprim.get_address_history(
    "mjQx3W3AcPTC73KiknrGNgt5K5YM7cffrx", 200, 100000
  ));*/
/*
016F2ABDCC44618ED5A3E1B28067E6BE6B50063C
"111142189204689714221316322517812810323019010780660"
*/
  //state.graceful_stop();
  // &std::io::stdout(), &std::io::stderr());
  
  ctrlc::set_handler(move || {
    println!("Do not signal. Stop by visiting /stop");
  }).expect("Error setting Ctrl-C handler");
	
  rocket::ignite().manage(state).mount("/", routes![hello, stop]).launch();

}
