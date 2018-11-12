use std;
use std::str::FromStr;
use tiny_ram_db::hashbrown;

use bitprim::errors::Error;
use bitprim::executor::Executor;
use bitprim::explorer::Received;
use bitprim::payment_address::PaymentAddress;
use tiny_ram_db::{PlainTable, Record};
use jsonapi::model::Query;

use models::address::Address;
use models::transaction::Transaction;
use models::database::Database;

pub trait Wallet: std::marker::Sized + Clone + std::fmt::Debug {
    type Utxo;
    type RA: Address;

    fn get_utxos(
        &self,
        exec: &Executor,
        addresses: hashbrown::HashSet<Record<Self::RA>>,
        limit: Option<u64>,
        maybe_since: Option<u64>,
    ) -> Vec<Self::Utxo> {
        let explorer = exec.explorer();

        let since = self.get_since(exec, maybe_since);

        self.get_transactions_for_wallet(
            limit.unwrap_or(10_000),
            since,
            addresses,
            |address, limit, since| explorer.address_unspents(address, limit, since),
            |received: Received, address| self.construct_utxo(received, address),
        )
    }

    fn get_incoming(
        &self,
        exec: &Executor,
        addresses: hashbrown::HashSet<Record<Self::RA>>,
        limit: Option<u64>,
        maybe_since: Option<u64>,
    ) -> Vec<Transaction> {
        let explorer = exec.explorer();

        let since = self.get_since(exec, maybe_since);

        self.get_transactions_for_wallet(
            limit.unwrap_or(10_000),
            since,
            addresses,
            |address, limit, since| explorer.address_incoming(address, limit, since),
            |received: Received, address| self.construct_transaction(received, address),
        )
    }

    fn construct_transaction(&self, received: Received, address: Record<Self::RA>) -> Transaction {
        Transaction::new(received, address.data.public())
    }

    fn construct_utxo(&self, received: Received, address: Record<Self::RA>) -> Self::Utxo;

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
        addresses: hashbrown::HashSet<Record<Self::RA>>,
        explorer_fn: E,
        tx: F,
    ) -> Vec<T>
    where
        E: Fn(PaymentAddress, u64, u64) -> Result<Vec<Received>, Error>,
        F: Fn(Received, Record<Self::RA>) -> T,
    {
        let mut result: Vec<T> = vec![];
        for record in addresses {
            if let Ok(valid_address) = PaymentAddress::from_str(&record.data.public()) {
                let vec_received = explorer_fn(valid_address, limit, since)
                    .expect("Not expecting failure on explore transaction!");

                for received in vec_received.into_iter() {
                    result.push(tx(received, record.clone()))
                }
            }
        }
        result
    }

    fn default_query() -> Query {
        Query::from_params(&format!(
            "include=[]&fields[{}]={}",
            Self::jsonapi_type(),
            "wallet"
        ))
    }

    fn jsonapi_type() -> &'static str;

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self>;
}
