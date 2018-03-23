#![feature(ptr_internals)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate libc;
extern crate rocket;
mod bitprim;
mod server_state;
mod wallet;
use rocket::State;
use server_state::ServerState;

#[cfg(test)] mod tests;

#[get("/")]
fn hello(node: State<ServerState>) -> String {
	//format!("{:?}", node.last_height())
	format!("{:?}", node.bitprim.get_address_history(
    "mjQx3W3AcPTC73KiknrGNgt5K5YM7cffrx", 200, 0
  ))
}

fn main() {
  let state = ServerState::new("/home/nubis/btc-testnet.cfg",
    &std::io::stdout(), &std::io::stderr());
	
  rocket::ignite().manage(state).mount("/", routes![hello]).launch();
}
