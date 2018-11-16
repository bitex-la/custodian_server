use models::plain_wallet::PlainUtxo;
use models::hd_wallet::HdUtxo;
use models::multisig_wallet::MultisigUtxo;

pub trait Utxo {
    fn id(&self) -> String;
}

impl Utxo for HdUtxo {
    fn id(&self) -> String {
        self.transaction.transaction_hash.clone()
    }
}

impl Utxo for PlainUtxo {
    fn id(&self) -> String {
        self.prev_hash.clone()
    }
}

impl Utxo for MultisigUtxo {
    fn id(&self) -> String {
        self.transaction.transaction_hash.clone()
    }
}
