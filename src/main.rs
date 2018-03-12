#![feature(ptr_internals)]
extern crate libc;
mod bitprim;
use bitprim::Bitprim;
use std::{thread, time, sync};

fn main() {
	let bitprim = sync::Arc::new(
		Bitprim::new("/home/nubis/btc-testnet.cfg",
			&std::io::stdout(), &std::io::stderr())
	);
	
	let handles : Vec<thread::JoinHandle<()>> = (0..3).map(|tn|{
		let prim = bitprim.clone();
		thread::spawn(move ||{
			for check in 0..3 {
				println!("Thread: {}. Check {}. Height {}", tn, check,
					prim.last_height().expect("Can't get last height"));
		    thread::sleep(time::Duration::from_millis(10000));
			}
		})
	}).collect();

	for h in handles {
		h.join().expect("could not join thread");
	}

	println!("Ok byebye");
}
