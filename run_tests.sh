#!/bin/bash

rm -rf tests/btc-testnet-files/database
echo "Copying prepared database"
cp -r tests/btc-testnet-files/prepared_database tests/btc-testnet-files/database
if [ $? -ne 0 ]
then
  echo "Failed to copy prepared database. Try ./prepare_tests.sh"
else
  echo "Ok all done"
  reset
  cargo test $1 -- --test-threads=1 --nocapture
fi
