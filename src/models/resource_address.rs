use models::address::Address;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceAddress<A: Address> {
    pub id: Option<usize>,
    pub data: A
}