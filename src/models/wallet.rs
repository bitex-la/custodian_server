use std::str::FromStr;

use bitprim::executor::Executor;
use bitprim::explorer::Received;
use bitprim::payment_address::PaymentAddress;

use models::resource_address::ResourceAddress;

pub trait Wallet {
    type Utxo;
    type A: ResourceAddress;

    fn get_utxos(&self, exec: &Executor, limit: Option<u64>, since: Option<u64>) -> Vec<Option<Self::Utxo>> {
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
                                Some(self.construct_utxo(received, address))
                            }).collect()
                        },
                        Err(err) => {
                            println!("{:?}", err);
                            vec![None]
                        }
                    }
                },
                Err(err)         => { 
                    println!("{:?}", err);
                    vec![None]
                }
            }
        }).collect()
    }

    fn construct_utxo(&self, received: Received, address: &Self::A) -> Self::Utxo;

    fn get_addresses<'a>(&'a self) -> &'a Vec<Self::A>;
}
