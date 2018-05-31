use bitprim::executor::Executor;

pub trait Wallet {
    type Utxo;

    fn get_utxos(&self, exec: &Executor) -> Vec<Option<Self::Utxo>>;
}
