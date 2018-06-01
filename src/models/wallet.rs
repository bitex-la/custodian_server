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

    fn get_utxos(&self, exec: &Executor, limit: Option<u64>, since: Option<u64>) -> Vec<Self::Utxo> {
        let explorer = exec.explorer();

        self.get_addresses().iter().flat_map(|address| {
            match PaymentAddress::from_str(&address.to_string()) {
                Ok(valid_address) => {
                    let last_height = exec.get_chain().get_last_height();
                    match explorer.address_unspents(valid_address,
                                                    limit.unwrap_or(10_000),
                                                    since.unwrap_or(last_height.unwrap_or(1_000) - 1_000)) {
                        Ok(vec_received) => {
                            vec_received.into_iter().map(|received| {
                                self.construct_utxo(received, address)
                            }).collect()
                        },
                        Err(err) => {
                            println!("{:?}", err);
                            vec![]
                        }
                    }
                },
                Err(err)         => { 
                    println!("{:?}", err);
                    vec![]
                }
            }
        }).collect()
    }

    fn construct_utxo(&self, received: Received, address: &Self::RA) -> Self::Utxo;

    fn get_addresses<'a>(&'a self) -> &'a Vec<Self::RA>;
}
