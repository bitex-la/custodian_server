extern crate bitprim;
extern crate custodian_server;

use std::fs::File;
use std::ops::Deref;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use bitprim::errors::*;
use bitprim::explorer::OpaqueCollection;
use bitprim::payment_address::PaymentAddress;
use bitprim::transaction::Transaction;
use bitprim::{Executor, ExitCode};
use custodian_server::models::plain_wallet::PlainWallet;
use custodian_server::models::*;
use custodian_server::server_state::ServerState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[cfg(feature = "btc")]
const CURRENCY: &str = "btc";

#[cfg(feature = "bch")]
const CURRENCY: &str = "bch";

#[cfg(feature = "ltc")]
const CURRENCY: &str = "ltc";

macro_rules! assert_ok {
    ($name:ident $body:block) => {
        #[test]
        fn $name() {
            let result: Result<()> = (|| {
                $body;
                Ok(())
            })();
            match result {
                Err(e) => assert!(false, format!("{}", e)),
                _ => assert!(true),
            }
        }
    };
}

fn build_test_executor() -> Result<Executor> {
    let f = File::create("/dev/null").unwrap();
    let exec = Executor::new(&format!("./tests/{}-testnet.cfg", CURRENCY), &f, &f);
    exec.initchain()?;
    Ok(exec)
}

fn build_500_blocks_state() -> Result<ServerState> {
    let f = File::create("/dev/null").unwrap();
    let state: ServerState =
        ServerState::new(&format!("./tests/{}-testnet.cfg", CURRENCY), &f, &f)?;
    while state.executor.get_chain().get_last_height()? < 500 {
        println!(
            "Syncing {:?}",
            state.executor.get_chain().get_last_height()?
        );
        sleep(Duration::new(1, 0));
    }
    Ok(state)
}

/*
assert_ok!{ gets_utxos_for_3_wallets {
  let state = build_500_blocks_state()?;
  let hd_wallet = PlainWallet{
    id: "incoming".to_string(),
    version: "1".to_string(),
    addresses: vec!["1".to_string(), "2".to_string(), "3".to_string()]
  };
  let wallets = state.wallets_lock();
}}
*/

assert_ok!{ runs_500_blocks_sync {
    let state = build_500_blocks_state()?;
    state.executor.stop()?;
    while !state.executor.is_stopped() {
      sleep(Duration::new(1,0));
    }
}}

assert_ok!{ runs_500_blocks_async {
    let exec = build_test_executor()?;
    exec.run(|exec, exit_code| {
      if exit_code != ExitCode::Success {
        assert!(false, format!("Async runner failed with {:?}", exit_code));
      }else{
        while exec.get_chain().get_last_height().expect("height fail") < 500 {
          sleep(Duration::new(1,0));
        }
      }
    });
}}

assert_ok!{ gets_last_height_async {
    let state = build_500_blocks_state()?;
    state.executor.run(|exec, _|{
      exec.get_chain().fetch_last_height(|_chain, exit, height|{
        println!("Async fetch last height: {}, {:?}", height, exit);
        assert!(height >= 500, "Height was not over 1000");
      })
    })
}}

assert_ok!{ gets_earliest_transaction_block {
  let state = build_500_blocks_state()?;
  let chain = state.executor.get_chain();
  let (block, _) = chain.get_block_by_height(429)?;
  let height = chain.get_block_height(block.hash())?;
  assert!(height == 429);
  assert!(block.hash().to_hex() ==
    "00000000e080223655db52d2c35a37f6aa17a3f2efefa6794fd9831374cff09f");
  assert!(block.deref().len() == 49);
}}

assert_ok!{ fetches_earliest_transaction_block {
  let state = build_500_blocks_state()?;
  let chain = state.executor.get_chain();
  chain.fetch_block_by_height(429, |new_chain, _, block, _height|{
    assert!(block.hash().to_hex() ==
      "00000000e080223655db52d2c35a37f6aa17a3f2efefa6794fd9831374cff09f");
    assert!(block.len() == 49);
    new_chain.fetch_block_height(block.hash(), |_, _, height:u64|{
      assert!(height == 429);
    });
  })
}}

assert_ok!{ gets_unspents_for_an_address {
  let state = build_500_blocks_state()?;
  let chain = state.executor.get_chain();
  let addr = PaymentAddress::from_str("mqETuaBY9Tiq1asdsehEyQgCHe34SrXQs9").unwrap();
  let hist = chain.get_history(addr, 1000, 1)?;
  assert!(hist.len() == 2);
  let first = hist.deref().get(0);
  println!("Point kind {:?}", first.point_kind());
  println!("Value {:?}", first.get_value_or_previous_checksum());
}}

/*
assert_ok!{ explores_incoming_funds_to_address {
}};

assert_ok!{ explores_utxos_for_address {
}};
*/
