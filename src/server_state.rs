use wallet::Wallet;
use bitprim::Bitprim;
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{ATOMIC_BOOL_INIT, AtomicBool, Ordering};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use std::process;
use rocket::outcome::Outcome;
use rocket::State;
use std::clone::Clone;

pub struct ServerState {
  pub wallets: Vec<Wallet>,
  pub bitprim: Bitprim,
  stopping: AtomicBool
}

impl ServerState {
  pub fn new<O,E>(config_path: &str, out: &O, err: &E) -> Self
    where O: AsRawFd, E: AsRawFd
  {
    Self{
      bitprim: Bitprim::new(config_path, out, err),
      wallets: vec![],
      stopping: AtomicBool::new(false)
    }
  }

  pub fn graceful_stop(&self){
    self.stopping.store(true, Ordering::Relaxed);
		self.bitprim.graceful_stop();
    process::exit(0);
  }
}

pub struct GuardedServerState<'r> {
	pub state: State<'r, ServerState>
}

impl<'a, 'r> FromRequest<'a, 'r> for GuardedServerState<'r> {
	type Error = ();

	#[inline(always)]
	fn from_request(request: &'a Request<'r>) -> request::Outcome<GuardedServerState<'r>, ()> {
		let state = request.guard::<State<ServerState>>()?;
		if state.stopping.load(Ordering::Relaxed) {
			Outcome::Failure((Status::ServiceUnavailable, ()))
		}else{
			Outcome::Success(GuardedServerState{state})
		}
	}
}

