use models::resource_address::ResourceAddress;

pub trait ResourceWallet<A: ResourceAddress> {
    fn raw_id(&self) -> Option<&String>;

    fn id(&self) -> i32 {
        self.raw_id().map(|x| x.parse::<i32>().unwrap_or(0) )
          .unwrap_or(0)
    }

    fn merge(self, newer: Self) -> Self;

    /*

    fn add_address(&mut self, address: A) -> Result<bool, String>;

    fn get_addresses(&self) -> Vec<A>;

    fn remove_address(&mut self, address: A) -> Result<bool, String>;
    */
}
