#!/bin/bash

rm -rf tests/bch-testnet-files/database
rm -rf tests/bch-testnet-files/log
reset
cargo +nightly test runs_500_blocks_sync -- --test-threads=1 --nocapture
if [ $? -ne 0 ]
then
  echo "Failed to run test. Database is likely invalid. Fix stuff and try again."
else
  echo "Copying prepared database"
  mv tests/bch-testnet-files/database tests/bch-testnet-files/prepared_database
  echo "Ok all done"
fi
