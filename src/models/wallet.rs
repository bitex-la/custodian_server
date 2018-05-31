use bitprim::executor::Executor;
use bitprim::explorer::Received;

pub trait Wallet {
    type Utxo;
    type A;

    fn get_utxos(&self, exec: &Executor) -> Vec<Option<Self::Utxo>>;

    fn construct_utxo(&self, received: Received, address: &Self::A) -> Self::Utxo;

    fn get_addresses<'a>(&'a self) -> &'a Vec<Self::A>;
}
