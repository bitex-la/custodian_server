use models::resource_address::ResourceAddress;

pub trait ResourceWallet<A: ResourceAddress> {
    fn raw_id(&self) -> Option<u64>;

    fn id(&self) -> u64 {
        self.raw_id().unwrap_or(0)
    }

    fn set_id(self, new_id: u64) -> Self;

    fn merge(self, newer: Self) -> Self;

    fn add_address(&mut self, address: A) -> Result<bool, String>;

    /*


    fn get_addresses(&self) -> Vec<A>;

    fn remove_address(&mut self, address: A) -> Result<bool, String>;
    */
}
