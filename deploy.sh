#!/bin/bash

CURRENCY=$1
USER=$2
IP=$3 

if [ "$CURRENCY" == "btc" ]
then
    echo "btc"
elif [ "$CURRENCY" == "bch" ]
then
    echo "bch"
elif [ "$CURRENCY" == "ltc" ]
then
    echo "ltc"
else
    echo "Currency not valid."
    exit
fi

cargo build --release --features $CURRENCY
trezor-agent $HOME/.ssh/config -- scp target/release/custodian_server $USER@$IP:/home/ubuntu/apps/custodian_server/$CURRENCY 
