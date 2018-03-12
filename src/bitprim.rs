use std::os::unix::io::AsRawFd;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr::Unique;
use std;

pub enum RawExecutor {}
unsafe impl std::marker::Send for RawExecutor {}

pub enum RawChain {}
unsafe impl std::marker::Send for RawChain {}

extern {
  fn executor_construct_fd(path: *const c_char, sout: c_int, serr: c_int)
    -> *mut RawExecutor;
  fn executor_destruct(exec: &RawExecutor);
  fn executor_initchain(exec: &RawExecutor) -> c_int;
  fn executor_run_wait(exec: &RawExecutor) -> c_int;
  fn executor_get_chain(exec: &RawExecutor) -> *mut RawChain;
  fn chain_get_last_height(chain: &RawChain, out_heigth: *mut c_uint) -> c_int;
}

pub struct Bitprim {
  exec: Unique<RawExecutor>
}

type BitprimError = c_int;

impl Bitprim {
  pub fn new<O,E>(config_path: &str, out: &O, err: &E) -> Bitprim
    where O: AsRawFd, E: AsRawFd
  {
    let path = CString::new(config_path).expect("Invalid config path");
    let exec = unsafe{
      executor_construct_fd(path.as_ptr(), out.as_raw_fd(), err.as_raw_fd())
    };
    unsafe {
      executor_initchain(&*exec);
      executor_run_wait(&*exec);
    };
    Self{exec: Unique::new(exec).unwrap() }
  }

  pub fn last_height(&self) -> Result<u32, BitprimError> {
    let chain = self.get_chain();
    let mut height = 0;
    let result = unsafe{ chain_get_last_height(&*chain, &mut height) };
    if result != 0 {
      Err(result)
    }else{
      Ok(height)
    }
  }
  
  fn get_chain(&self) -> *mut RawChain {
    unsafe { executor_get_chain(self.exec.as_ref()) }
  }
}

impl Drop for Bitprim {
	fn drop(&mut self) {
		unsafe { executor_destruct(self.exec.as_ref()) };
	}
}
