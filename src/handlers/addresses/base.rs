use handlers::handler::{parse_to_value, JsonResult};
use jsonapi::model::*;
use models::resource_wallet::ResourceWallet;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;

pub trait AddressHandler: ResourceWallet {
    fn address_index(state: &ServerState, id: u64) -> JsonResult {
        let mut wallets = state.wallets_lock();
        let haystack = Self::collection_from_wallets(&mut wallets);

        match haystack.iter_mut().find(|wallet| wallet.id() == id) {
            Some(maybe_wallet) => parse_to_value(vec_to_jsonapi_document_with_query(
                maybe_wallet.get_addresses().to_vec(),
                &Self::addresses_query(),
            )),
            None => Err(status::Custom(
                Status::NotFound,
                format!("Wallet {:?} Not Found", id),
            )),
        }
    }

    fn address_create(state: &ServerState, id: u64, address: Self::A) -> JsonResult {
        let mut wallets = state.wallets_lock();
        let haystack = Self::collection_from_wallets(&mut wallets);

        match haystack.iter().position(|wallet| wallet.id() == id) {
            Some(wallet_position) => {
                match haystack[wallet_position].find_address_position(&address) {
                    Some(_) => Err(status::Custom(
                        Status::InternalServerError,
                        format!("Duplicate address {:?}", address),
                    )),
                    None => {
                        haystack[wallet_position].add_address(address);
                        match haystack[wallet_position].get_addresses().last() {
                            Some(last_address) => {
                                parse_to_value(last_address.to_jsonapi_document())
                            }
                            None => Err(status::Custom(
                                Status::InternalServerError,
                                "Problem adding address".to_string(),
                            )),
                        }
                    }
                }
            }
            None => Err(status::Custom(
                Status::NotFound,
                format!("Wallet {:?} Not Found", id),
            )),
        }
    }

    fn address_destroy(state: &ServerState, id: u64, address: Self::A) -> JsonResult {
        let mut wallets = state.wallets_lock();
        let haystack = Self::collection_from_wallets(&mut wallets);

        match haystack.iter().position(|wallet| wallet.id() == id) {
            Some(value) => match haystack[value].find_address_position(&address) {
                Some(position) => {
                    haystack[value].remove_address(position);
                    parse_to_value(address.to_jsonapi_document())
                }
                None => Err(status::Custom(
                    Status::NotFound,
                    format!("Address not found {:?}", address),
                )),
            },
            None => Err(status::Custom(
                Status::NotFound,
                format!("Wallet with id {:?} Not Found", id),
            )),
        }
    }
}

impl<R: ResourceWallet> AddressHandler for R {}
