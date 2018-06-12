#!/bin/bash

rm -rf tests/btc-testnet-files/database
rm -rf tests/btc-testnet-files/log
reset
cargo test runs_500_blocks_sync -- --test-threads=1 --nocapture
if [ $? -ne 0 ]
then
  echo "Failed to run test. Database is likely invalid. Fix stuff and try again."
else
  echo "Copying prepared database"
  mv tests/btc-testnet-files/database tests/btc-testnet-files/prepared_database
  echo "Ok all done"
fi
