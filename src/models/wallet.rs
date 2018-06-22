use std;
use std::str::FromStr;

use bitprim::chain::Chain;
use bitprim::errors::Error;
use bitprim::executor::Executor;
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
            limit.unwrap_or(10_000),
            since,
            |address, limit, since| explorer.address_unspents(address, limit, since),
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
            limit.unwrap_or(10_000),
            since,
            |address, limit, since| explorer.address_incoming(address, limit, since),
            |received: Received, address| self.construct_transaction(received, address),
        )
    }

    fn construct_transaction(&self, received: Received, address: &Self::RA) -> Transaction {
        Transaction::new(received, address.to_string())
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

    fn get_transactions_for_wallet<T, F, E>(
        &self,
        limit: u64,
        since: u64,
        explorer_fn: E,
        tx: F,
    ) -> Vec<T>
    where
        E: Fn(PaymentAddress, u64, u64) -> Result<Vec<Received>, Error>,
        F: Fn(Received, &Self::RA) -> T,
    {
        let mut result: Vec<T> = vec![];
        for address in self.get_addresses() {
            if let Ok(valid_address) = PaymentAddress::from_str(&address.to_string()) {
                let vec_received = explorer_fn(valid_address, limit, since)
                    .expect("Not expecting failure on explore transaction!");

                for received in vec_received {
                    result.push(tx(received, address))
                }
            }
        }
        result
    }
}
