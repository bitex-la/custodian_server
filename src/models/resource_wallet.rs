use models::resource_address::ResourceAddress;

pub trait ResourceWallet<A: ResourceAddress> {
    fn id(&self) -> i32;

    fn add_address(&mut self, address: A) -> Result<bool, String>;

    fn get_addresses(&self) -> Vec<A>;

    fn remove_address(&mut self, address: A) -> Result<bool, String>;
}
