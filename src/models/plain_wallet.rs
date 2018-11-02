use bitprim::explorer::Received;
use jsonapi::model::*;
use models::database::Database;
use models::plain_address::PlainAddress;
use models::resource_transaction::JsonApiModelTransaction;
use models::wallet::Wallet;
use tiny_ram_db::PlainTable;
use data_guards::FromJsonApiDocument;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromForm)]
pub struct PlainWallet {
    pub version: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainUtxo {
    pub prev_hash: String,
    pub prev_index: u32,
    pub address: PlainAddress,
    pub amount: u64,
}

impl JsonApiModelTransaction for PlainUtxo {
    fn jsonapi_type() -> &'static str {
        "plain_utxo"
    }
}

impl Wallet for PlainWallet {
    type Utxo = PlainUtxo;
    type RA = PlainAddress;

    fn construct_utxo(&self, received: Received, address: &PlainAddress) -> Self::Utxo {
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

    fn wallets_from_database<'a>(database: &'a mut Database) -> &'a mut PlainTable<Self> {
        &mut database.plain_wallets
    }
}

impl ToJsonApi for PlainWallet {
    const TYPE : &'static str = "plain_wallets";

		fn attributes(&self, _fields: &QueryFields) -> ResourceAttributes {
				hashmap!{
						"version" => serde_json::to_value(self.version).unwrap()
						"label" => serde_json::to_value(self.label).unwrap()
				}
		}
}

impl FromJsonApiDocument for PlainWallet {
    const TYPE : &'static str = "plain_wallets";

    fn from_json_api_resource(resource: Resource, _db: Database) -> Result<Self, String> {
        Ok(PlainWallet{
            version: Self::attribute(&resource, "version")?,
            label: Self::attribute(&resource, "label")?
        })
    }
}

