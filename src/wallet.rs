#[derive(Debug)]
pub struct Wallet {
  pub id: String,
  pub version: String,
  pub addresses: Vec<String>
}
