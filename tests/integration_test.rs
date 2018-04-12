extern crate custodian_server;

use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use bitprim::{Executor, ExitCode};
use bitprim::errors::*;
use bitprim::transaction::Transaction;
use bitprim::payment_address::PaymentAddress;
use std::sync::{Arc,Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

macro_rules! assert_ok {
  ($name:ident $body:block) => (
    #[test]
    fn $name(){
      let result : Result<()>= (||{ $body; Ok(()) })();
      match result {
        Err(e) => assert!(false, format!("{}", e)),
        _ => assert!(true)
      }
    }
  )
}

fn build_500_blocks_state() -> Result<ServerState> {
  let f = File::create("/dev/null").unwrap();
  let state = ServerState::new("./tests/btc-testnet.cfg", &f, &f)?;
  while state.executor.get_chain().get_last_height()? < 500 {
    println!("Syncing {:?}", exec.get_chain().get_last_height()?);
    sleep(Duration::new(1,0));
  }
  Ok(state)
}

assert_ok!{ gets_utxos_for_3_wallets {
  let state = build_500_blocks_state()?;
  let hd_wallet = HdWallet{
    id: "incoming",
    version: "1"
  };
  let wallets = state.wallets_lock();
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
    let exec = build_500_blocks_executor()?;
    exec.run(|exec, _|{
      exec.get_chain().fetch_last_height(|_chain, exit, height|{
        println!("Async fetch last height: {}, {:?}", height, exit);
        assert!(height >= 500, "Height was not over 1000");
      })
    })
}}

assert_ok!{ gets_earliest_transaction_block {
  let exec = build_500_blocks_executor()?;
  let chain = exec.get_chain();
  let (block, _) = chain.get_block_by_height(429)?;
  let height = chain.get_block_height(block.hash())?;
  assert!(height == 429);
  assert!(block.hash().to_hex() ==
    "00000000e080223655db52d2c35a37f6aa17a3f2efefa6794fd9831374cff09f");
  assert!(block.transaction_count() == 49);
}}

assert_ok!{ fetches_earliest_transaction_block {
  let exec = build_500_blocks_executor()?;
  let chain = exec.get_chain();
  chain.fetch_block_by_height(429, |new_chain, _, block, _height|{
    assert!(block.hash().to_hex() ==
      "00000000e080223655db52d2c35a37f6aa17a3f2efefa6794fd9831374cff09f");
    assert!(block.transaction_count() == 49);
    new_chain.fetch_block_height(block.hash(), |_, _, height:u64|{
      assert!(height == 429);
    });
  })
}}

assert_ok!{ gets_unspents_for_an_address {
  let exec = build_500_blocks_executor()?;
  let chain = exec.get_chain();
  let addr = PaymentAddress::from_str("mqETuaBY9Tiq1asdsehEyQgCHe34SrXQs9");
  let hist = chain.get_history(addr, 1000, 1)?;
  assert!(hist.count() == 2);
  let first = hist.nth(0);
  println!("Point kind {:?}", first.get_point_kind());
  println!("Value {:?}", first.get_value_or_previous_checksum());
}}

/*
assert_ok!{ explores_incoming_funds_to_address {
}};

assert_ok!{ explores_utxos_for_address {
}};
*/

