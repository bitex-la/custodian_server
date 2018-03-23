use std::os::unix::io::AsRawFd;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr::Unique;
use std;
use libc::{uint32_t, size_t};
use std::slice;

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
  fn chain_get_history(chain: &RawChain, address: *const c_char, limit: c_int,
    from_height: c_int, out_history: *mut c_int) -> c_int;
	fn chain_history_compact_list_count(history: &c_int) -> u64;
	fn chain_history_compact_list_nth(history: &c_int, n: i32, raw_item: &mut c_int) -> c_int;
	fn chain_history_compact_get_point_kind(raw_item: &c_int) -> u64;
	fn chain_history_compact_get_height(raw_item: &c_int) -> u32;
	fn chain_history_compact_get_value_or_previous_checksum(raw_item: &c_int) -> u64;
	fn chain_history_compact_get_point(raw_item: &c_int) -> *const c_int;
	fn chain_point_get_index(point: *const c_int) -> u32;
	fn chain_point_get_hash_out(point: *const c_int, bytes: *mut c_int);
}

type BitprimError = c_int;

pub struct Bitprim {
  exec: Unique<RawExecutor>
}

pub fn to_hex_string(bytes: &[i32]) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
  strs.join(" ")
}

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

	pub fn get_address_history(&self, address: &str, limit: i32, from_height: i32)
		-> Result<Vec<AddressHistoryItem>, BitprimError>
	{
    let chain = self.get_chain();
    let c_address = CString::new(address).expect("Invalid address");

	  let mut history = 0;

		let result = unsafe{
			chain_get_history(&*chain, c_address.as_ptr(), limit, from_height, &mut history)
		};

    if result != 0 { return Err(result) }

		let count = unsafe{ chain_history_compact_list_count(&history) };

		let mut items = vec![];
		for i in 0..count {
			let mut raw_item = 0;
			let result = unsafe {
				chain_history_compact_list_nth(&history, i as i32, &mut raw_item)
			};
			if result != 0 { return Err(result) }

      let point_kind = unsafe{ chain_history_compact_get_point_kind(&raw_item) };
			
			if point_kind == 0 { // 0 is output, 1 is input/spend
				let block_height = unsafe{ chain_history_compact_get_height(&raw_item) };
				let satoshis = unsafe{
					chain_history_compact_get_value_or_previous_checksum(&raw_item)
				};

				let point = unsafe{ chain_history_compact_get_point(&raw_item) };
				let position = unsafe{ chain_point_get_index(point) };

				let transaction = unsafe{
					let mut bytes = 0;
					chain_point_get_hash_out(point, &mut bytes);
					let s = slice::from_raw_parts(&bytes, 32);
					to_hex_string(s)
				};

				items.push(
					AddressHistoryItem {transaction, satoshis, position, block_height}
				)
			}
		}
		Ok(items)
	}
  
  fn get_chain(&self) -> *mut RawChain {
		// Get Chain needs to be dropped later!
    unsafe { executor_get_chain(self.exec.as_ref()) }
  }
}

impl Drop for Bitprim {
	fn drop(&mut self) {
		unsafe { executor_destruct(self.exec.as_ref()) };
	}
}

#[derive(Debug)]
pub struct AddressHistoryItem {
	transaction: String,
	satoshis: u64,
	position: u32,
	block_height: u32,
}
