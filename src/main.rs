#![feature(ptr_internals)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate libc;
extern crate rocket;
mod bitprim;
use bitprim::Bitprim;
use std::{thread, time, sync};
use rocket::State;

#[cfg(test)] mod tests;

#[get("/")]
fn hello(node: State<bitprim::Bitprim>) -> String {
	format!("{:?}", node.last_height())
}

fn main() {
  let bitprim = Bitprim::new("/home/nubis/btc-testnet.cfg",
    &std::io::stdout(), &std::io::stderr());
	
  rocket::ignite().manage(bitprim).mount("/", routes![hello]).launch();
}
