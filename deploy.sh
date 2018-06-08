#!/bin/bash

CURRENCY=$1

if [ "$CURRENCY" == "btc" ]
then
    echo "btc"
elif [ "$CURRENCY" == "bch" ]
then
    echo "bch"
else
    echo "Currency not valid."
    exit
fi

cargo clean
cargo +nightly build --release
trezor-agent $HOME/.ssh/config -- scp target/release/custodian_server user@200.89.175.55:/tmp/custodian_server 

trezor-agent $HOME/.ssh/config -- ssh user@200.89.175.55 screen -d -m "sudo su -c 'cp /tmp/custodian_server /var/apps/custodian_server/$CURRENCY/' bitex"
