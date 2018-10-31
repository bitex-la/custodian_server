use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;
use models::address::Address;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceAddress<A: Address, W: Wallet> {
    pub id: Option<usize>,
    pub address: A,
    pub wallet: ResourceWallet<W>
}