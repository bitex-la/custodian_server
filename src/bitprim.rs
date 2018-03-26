use std::os::unix::io::AsRawFd;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint};
use libc::{strlen};
use std::ptr::Unique;
use std;
use libc::{uint32_t, size_t};
use std::slice;
use std::mem;
use std::str;

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

  fn chain_get_history(chain: &RawChain, address: &c_int, limit: c_int,
    from_height: c_int, out_history: *mut c_int) -> c_int;

	fn chain_history_compact_list_count(history: &c_int) -> i32;

	fn chain_history_compact_list_nth(history: &c_int, n: i32, raw_item: &mut c_uint) -> c_int;
	fn chain_history_compact_get_point_kind(raw_item: &c_uint) -> u64;
	fn chain_history_compact_get_height(raw_item: &c_uint) -> u32;
	fn chain_history_compact_get_value_or_previous_checksum(raw_item: &c_uint) -> u64;
	fn chain_history_compact_get_point(raw_item: &c_uint) -> *const c_int;
	fn chain_point_get_index(point: *const c_int) -> u32;
	fn chain_point_get_hash_out(point: *const c_int, bytes: *mut c_int);

  fn chain_payment_address_construct_from_string(hex: *const c_char) -> *mut c_int;
  fn chain_payment_address_encoded(addr: &c_int) -> *mut c_char;
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

pub fn to_a_string(bytes: &[c_char]) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:?}", b))
                               .collect();
  strs.join("")
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
    let b_address = unsafe{
      chain_payment_address_construct_from_string(c_address.as_ptr())
    };

    let debug_b_address = unsafe{
      let encoded = chain_payment_address_encoded(&*b_address);
      if encoded.is_null() { panic!("holy shit was null") }
      CString::from_raw(encoded)
    };
    println!("Debug b address is: {:?}", debug_b_address);

	  let mut history = unsafe{ mem::uninitialized() };

		let result = unsafe{
			chain_get_history(&*chain, &*b_address, limit, from_height, &mut history)
		};

    if result != 0 { return Err(result) }

		let count = unsafe{ chain_history_compact_list_count(&history) };
    println!("Got history has {:?} items", count);

		let mut items = vec![];
		for i in 0..count {
      println!("Getting {:?} item for history", i);
			let mut raw_item = unsafe{ mem::uninitialized() };

			let result = unsafe {
				chain_history_compact_list_nth(&history, i as i32, &mut raw_item)
			};
			if result != 0 { return Err(result) }

      println!("Getting compact history");
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

  pub fn graceful_stop(&self){
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
