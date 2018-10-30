use bitprim::errors::*;
use bitprim::executor::executor_destruct;
use bitprim::Executor;
use models::database::Database;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::State;
use std::os::unix::io::AsRawFd;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard};

pub struct ServerState {
    pub executor: Executor,
    pub database: Mutex<Database>,
    stopping: AtomicBool,
}

impl ServerState {
    pub fn new<O, E>(config_path: &str, out: &O, err: &E) -> Result<Self>
    where
        O: AsRawFd,
        E: AsRawFd,
    {
        let exec = Executor::new(config_path, out, err);
        exec.initchain()?;
        exec.run_wait()?;

        Ok(Self {
            executor: exec,
            database: Mutex::new(Database::new()),
            stopping: AtomicBool::new(false),
        })
    }

    pub fn database_lock(&self) -> MutexGuard<Database> {
        self.database.lock().expect("The lock has been poisoned")
    }

    pub fn graceful_stop(&self) {
        /* Due to how broken rocket's graceful shutdown is, we need to
         * do some low level cleanups in the executor and then call process::exit
         * In an ideal world, you would be able to tell rocket's main thread to stop.
         * Then program shutdown should follow, including all destructors.
         */
        self.stopping.store(true, Ordering::Relaxed);
        unsafe { executor_destruct(self.executor.raw) }
        process::exit(0);
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &'r ServerState {
    type Error = ();

    #[inline(always)]
    fn from_request(request: &'a Request<'r>) -> request::Outcome<&'r ServerState, ()> {
        let state = request.guard::<State<ServerState>>()?;
        if state.stopping.load(Ordering::Relaxed) {
            Outcome::Failure((Status::ServiceUnavailable, ()))
        } else {
            Outcome::Success(state.inner())
        }
    }
}
