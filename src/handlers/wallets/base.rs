use jsonapi::model::*;
use models::resource_wallet::ResourceWallet;
use rocket::http::Status;
use rocket::response::status;
use server_state::ServerState;
use handlers::handler::{JsonResult, parse_to_value};

pub trait WalletHandler: ResourceWallet {
    fn index(state: &ServerState) -> JsonResult {
        let mut wallets = state.wallets_lock();
        let all = Self::collection_from_wallets(&mut wallets);

        parse_to_value(vec_to_jsonapi_document_with_query(
            all.clone(),
            &Self::default_query()))
    }

    fn show(state: &ServerState, id: u64) -> JsonResult {
        let mut wallets = state.wallets_lock();
        let haystack = Self::collection_from_wallets(&mut wallets);
        let maybe_wallet = &haystack.iter().find(|&wallet| wallet.id() == id);

        match maybe_wallet {
            Some(wallet) => parse_to_value(wallet.to_jsonapi_document_with_query(&Self::default_query())),
            None => Err(status::Custom(Status::NotFound, format!("{:?}", id))),
        }
    }

    fn create(state: &ServerState, new: Self) -> JsonResult {
        let mut wallets = state.wallets_lock();
        let haystack = Self::collection_from_wallets(&mut wallets);

        if haystack
            .iter()
            .find(|&wallet| wallet.id() == new.id())
            .is_some()
        {
            Err(status::Custom(
                Status::InternalServerError,
                format!("Wallet with id {:?} is duplicated", new.id()),
            ))
        } else {
            let last_id = haystack.last().map(ResourceWallet::id).unwrap_or(0);
            haystack.push(new.set_auto_id_if_needed(last_id));
            match haystack.last() {
                Some(value) => parse_to_value(value.to_jsonapi_document()),
                None => Err(status::Custom(Status::NotFound, format!("No last wallet"))),
            }
        }
    }

    fn update(state: &ServerState, id: u64, new: Self) -> JsonResult {
        let mut wallets = state.wallets_lock();
        let haystack = Self::collection_from_wallets(&mut wallets);

        let maybe_position = &haystack.iter().position(|ref wallet| wallet.id() == id);

        match maybe_position {
            Some(position) => {
                let old_item = haystack.swap_remove(*position);
                let new_item = old_item.merge(new);
                haystack.push(new_item);
                parse_to_value(&haystack.last())
            }
            None => Err(status::Custom(Status::NotFound, format!("{:?}", id))),
        }
    }

    fn destroy(state: &ServerState, id: u64) -> JsonResult {
        let mut wallets = state.wallets_lock();
        let haystack = Self::collection_from_wallets(&mut wallets);
        let maybe_position = &haystack.iter().position(|ref wallet| wallet.id() == id);

        match maybe_position {
            Some(position) => {
                let old = haystack.swap_remove(*position);
                parse_to_value(&old)
            }
            None => Err(status::Custom(Status::NotFound, format!("{:?}", id))),
        }
    }
}

impl<R: ResourceWallet> WalletHandler for R {}
