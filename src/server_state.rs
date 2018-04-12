use bitprim::Executor;
use bitprim::errors::*;
use bitprim::executor::executor_destruct;
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use std::process;
use rocket::outcome::Outcome;
use rocket::State;
use std::sync::{Mutex, MutexGuard};
use wallet::Wallet;

pub struct ServerState {
  pub executor: Executor,
  wallets: Mutex<HashMap<String, Box<Wallet>>>,
  stopping: AtomicBool
}

impl ServerState {
  pub fn new<O,E>(config_path: &str, out: &O, err: &E) -> Result<Self>
    where O: AsRawFd, E: AsRawFd
  {
    let exec = Executor::new(config_path, out, err);
    exec.initchain()?;
    exec.run_wait()?;
    Self{
      executor: exec,
      wallets: Mutex::new(HashMap::new()),
      stopping: AtomicBool::new(false)
    }
  }

  pub fn wallets_lock(&self) -> MutexGuard<Vec<Wallet>> {
    self.wallets.lock().unwrap()
  }

  pub fn graceful_stop(&self){
    /* Due to how broken rocket's graceful shutdown is, we need to 
     * do some low level cleanups in the executor and then call process::exit
     * In an ideal world, you would be able to tell rocket's main thread to stop.
     * Then program shutdown should follow, including all destructors.
     */
    self.stopping.store(true, Ordering::Relaxed);
    unsafe{ executor_destruct(self.executor.raw) }
    process::exit(0);
  }
}

impl<'a,'r> FromRequest<'a, 'r> for &'r ServerState {
	type Error = ();

	#[inline(always)]
	fn from_request(request: &'a Request<'r>) -> request::Outcome<&'r ServerState, ()> {
		let state = request.guard::<State<ServerState>>()?;
		if state.stopping.load(Ordering::Relaxed) {
			Outcome::Failure((Status::ServiceUnavailable, ()))
		}else{
			Outcome::Success(state.inner())
		}
	}
}
