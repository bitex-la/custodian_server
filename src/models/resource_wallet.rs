use models::wallet::Wallet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceWallet<W: Wallet> {
    pub id: Option<usize>,
    pub wallet: W
}