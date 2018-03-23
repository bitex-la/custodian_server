use wallet::Wallet;
use bitprim::Bitprim;
use std::os::unix::io::AsRawFd;

pub struct ServerState {
  pub wallets: Vec<Wallet>,
  pub bitprim: Bitprim,
}

impl ServerState {
  pub fn new<O,E>(config_path: &str, out: &O, err: &E) -> Self
    where O: AsRawFd, E: AsRawFd
  {
    Self{bitprim: Bitprim::new(config_path, out, err), wallets: vec![]}
  }
}
