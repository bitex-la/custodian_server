pub trait ResourceWallet {
    fn id(&self) -> i32;
    fn add_address<A>(&self, address: A);
}
