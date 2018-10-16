rm -rf tests/bch-testnet-files/database
echo "Copying prepared database"
cp -r tests/bch-testnet-files/prepared_database tests/bch-testnet-files/database
if [ $? -ne 0 ]
then
  echo "Failed to copy prepared database. Try ./prepare_tests.sh"
else
  echo "Ok all done"
fi

