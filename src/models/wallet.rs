use std::str::FromStr;
use std;

use jsonapi::model::*;
use bitprim::explorer::Explorer;
use bitprim::executor::Executor;
use bitprim::explorer::Received;
use bitprim::payment_address::PaymentAddress;

use models::resource_address::ResourceAddress;

pub trait Wallet: std::marker::Sized + JsonApiModel + Clone + std::fmt::Debug {
    type Utxo: JsonApiModel;
    type RA: ResourceAddress;

    fn get_utxos(&self, exec: &Executor, limit: Option<u64>, maybe_since: Option<u64>) -> Vec<Self::Utxo> {
        let explorer = exec.explorer();

        let since = self.get_since(exec, maybe_since);

        self.get_transactions_for_wallet(explorer, limit.unwrap_or(10_000), since, |tx, add| { self.construct_utxo(tx, add) })
    }

    fn get_incoming(&self, exec: &Executor, limit: Option<u64>, maybe_since: Option<u64>) -> Vec<Received> {
        let explorer = exec.explorer();

        let since = self.get_since(exec, maybe_since);

        let mut result: Vec<Received> = vec![];
        for address in self.get_addresses() {
            if let Ok(valid_address) = PaymentAddress::from_str(&address.to_string()) {
                let vec_received = explorer.address_incoming(valid_address, limit.unwrap_or(10_000), since)
                    .expect("Not expecting failure on explore address unspent!");

                for received in vec_received {
                    result.push(received)
                }
            }
        }
        result
    }

    fn construct_utxo(&self, received: Received, address: &Self::RA) -> Self::Utxo;

    fn get_addresses<'a>(&'a self) -> &'a Vec<Self::RA>;

    fn get_since(&self, exec: &Executor, maybe_since: Option<u64>) -> u64 {
        maybe_since.unwrap_or_else(||{
            let height = exec.get_chain().get_last_height()
                .expect("Not expecting failure for last_height");
            if height > 1_000 { height - 1_000 } else { 1 }
        })
    }

    fn get_transactions_for_wallet<T, F>(&self, explorer: Explorer, limit: u64, since: u64, tx: F) -> Vec<T> 
        where F: Fn(Received, &Self::RA) -> T {
            let mut result: Vec<T> = vec![];
            for address in self.get_addresses() {
                if let Ok(valid_address) = PaymentAddress::from_str(&address.to_string()) {
                    let vec_received = explorer.address_unspents(valid_address, limit, since)
                        .expect("Not expecting failure on explore address unspent!");

                    for received in vec_received {
                        result.push(tx(received, address))
                    }
                }
            }
            result
        }
}
