use std::str::FromStr;
use std;

use jsonapi::model::*;
use bitprim::executor::Executor;
use bitprim::explorer::Received;
use bitprim::payment_address::PaymentAddress;

use models::resource_address::ResourceAddress;

pub trait Wallet: std::marker::Sized + JsonApiModel + Clone + std::fmt::Debug {
    type Utxo: JsonApiModel;
    type RA: ResourceAddress;

    fn get_utxos(&self, exec: &Executor, limit: Option<u64>, maybe_since: Option<u64>) -> Vec<Self::Utxo> {
        let explorer = exec.explorer();

        let since = maybe_since.unwrap_or_else(||{
            let height = exec.get_chain().get_last_height()
                .expect("Not expecting failure for last_height");
            if height > 1_000 { height - 1_000 } else { 1 }
        });

        let mut result: Vec<Self::Utxo> = vec![];
        for address in self.get_addresses() {
            if let Ok(valid_address) = PaymentAddress::from_str(&address.to_string()) {
                let vec_received = explorer.address_unspents(valid_address, limit.unwrap_or(10_000), since)
                    .expect("Not expecting failure on explore address unspent!");

                for received in vec_received {
                    result.push(self.construct_utxo(received, address))
                }
            }
        }
        result
    }

    fn construct_utxo(&self, received: Received, address: &Self::RA) -> Self::Utxo;

    fn get_addresses<'a>(&'a self) -> &'a Vec<Self::RA>;
}
