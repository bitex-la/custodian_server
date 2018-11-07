use serde_json;
use bitprim::explorer::Received;
use jsonapi::model::*;
use models::database::Database;
use models::plain_address::PlainAddress;
use models::wallet::Wallet;
use serializers::{FromJsonApi, ToJsonApi};
use tiny_ram_db::Record;
use tiny_ram_db::PlainTable;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromForm)]
pub struct PlainWallet {
    pub version: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainUtxo {
    pub prev_hash: String,
    pub prev_index: u32,
    pub address: Record<PlainAddress>,
    pub amount: u64,
}

impl ToJsonApi for PlainUtxo {
    const TYPE : &'static str = "plain_utxos";

    fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
        hashmap!{
            "prev_hash".to_string() => serde_json::to_value(&self.prev_hash).unwrap(),
            "prev_index".to_string() => serde_json::to_value(&self.prev_index).unwrap(),
            "amount".to_string() => serde_json::to_value(&self.amount).unwrap()
        }
    }

    fn relationships(&self, _fields: &QueryFields) -> Option<Relationships> {
        Some(hashmap!{
            "address".to_string() => Self::has_one("plain_addresses", self.address.id),
        })
    }

    fn included(&self, _fields: &Vec<String>) -> Option<Resources> {
        Some(vec![self.address.data.to_jsonapi_resource(self.address.id).0])
    }
}

impl Wallet for PlainWallet {
    type Utxo = PlainUtxo;
    type RA = PlainAddress;

    fn construct_utxo(&self, received: Received, address: Record<PlainAddress>) -> Self::Utxo {
        PlainUtxo {
            prev_hash: received.transaction_hash.to_hex(),
            prev_index: received.position,
            address: address.clone(),
            amount: received.satoshis,
        }
    }

    fn jsonapi_type() -> &'static str {
        "plain_wallet"
    }

    fn wallets_from_database<'a>(database: &'a mut Database)
        -> &'a mut PlainTable<Self> {
            &mut database.plain_wallets
        }
}

impl ToJsonApi for PlainWallet {
    const TYPE : &'static str = "plain_wallets";

    fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
        hashmap!{
            "version".to_string() => serde_json::to_value(&self.version).unwrap(),
            "label".to_string() => serde_json::to_value(&self.label).unwrap()
        }
    }
}

impl FromJsonApi for PlainWallet {
    const TYPE : &'static str = "plain_wallets";

    fn from_json_api_resource(resource: Resource, _db: Database) -> Result<Self, String> {
        Ok(PlainWallet{
            version: Self::attribute(&resource, "version")?,
            label: Self::attribute(&resource, "label")?
        })
    }
}

