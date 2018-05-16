use models::resource_address::ResourceAddress;

pub trait ResourceWallet<A: ResourceAddress> {
    fn id(&self) -> i32;

    fn add_address(&mut self, address: A);
}
