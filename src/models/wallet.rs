use std;
use std::str::FromStr;

use bitprim::executor::Executor;
use bitprim::explorer::Explorer;
use bitprim::explorer::Received;
use bitprim::payment_address::PaymentAddress;
use jsonapi::model::*;

use models::resource_address::ResourceAddress;
use models::transaction::Transaction;

pub trait Wallet: std::marker::Sized + JsonApiModel + Clone + std::fmt::Debug {
    type Utxo: JsonApiModel;
    type RA: ResourceAddress;

    fn get_utxos(
        &self,
        exec: &Executor,
        limit: Option<u64>,
        maybe_since: Option<u64>,
    ) -> Vec<Self::Utxo> {
        let explorer = exec.explorer();

        let since = self.get_since(exec, maybe_since);

        self.get_transactions_for_wallet(
            explorer,
            limit.unwrap_or(10_000),
            since,
            |received: Received, address| self.construct_utxo(received, address),
        )
    }

    fn get_incoming(
        &self,
        exec: &Executor,
        limit: Option<u64>,
        maybe_since: Option<u64>,
    ) -> Vec<Transaction> {
        let explorer = exec.explorer();

        let since = self.get_since(exec, maybe_since);

        self.get_transactions_for_wallet(
            explorer,
            limit.unwrap_or(10_000),
            since,
            |received: Received, address| self.construct_transaction(received, address),
        )
    }

    fn construct_transaction(&self, received: Received, address: &Self::RA) -> Transaction {
        Transaction {
            id: Some(format!(
                "{}-{}",
                received.transaction_hash, received.position
            )),
            satoshis: received.satoshis,
            transaction_hash: received.transaction_hash,
            position: received.position,
            is_spent: received.is_spent,
            block_height: received.block_height,
            address: address.to_string(),
        }
    }

    fn construct_utxo(&self, received: Received, address: &Self::RA) -> Self::Utxo;

    fn get_addresses<'a>(&'a self) -> &'a Vec<Self::RA>;

    fn get_since(&self, exec: &Executor, maybe_since: Option<u64>) -> u64 {
        maybe_since.unwrap_or_else(|| {
            let height = exec
                .get_chain()
                .get_last_height()
                .expect("Not expecting failure for last_height");
            if height > 1_000 {
                height - 1_000
            } else {
                1
            }
        })
    }

    fn get_transactions_for_wallet<T, F>(
        &self,
        explorer: Explorer,
        limit: u64,
        since: u64,
        tx: F,
    ) -> Vec<T>
    where
        F: Fn(Received, &Self::RA) -> T,
    {
        let mut result: Vec<T> = vec![];
        for address in self.get_addresses() {
            if let Ok(valid_address) = PaymentAddress::from_str(&address.to_string()) {
                let vec_received = explorer
                    .address_unspents(valid_address, limit, since)
                    .expect("Not expecting failure on explore address unspent!");

                for received in vec_received {
                    result.push(tx(received, address))
                }
            }
        }
        result
    }
}
