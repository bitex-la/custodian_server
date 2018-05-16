#[macro_export]
macro_rules! from_data_wallet {
    ( $wallet_type:ty ) => {
        impl FromData for $wallet_type {
            type Error = String;

            fn from_data(_: &Request, data: Data) -> data::Outcome<Self, String> {
                let mut string_wallets = String::new();
                if let Err(e) = data.open().read_to_string(&mut string_wallets) {
                    return Failure((Status::InternalServerError, format!("{:?}", e)));
                }

                let raw_json: JsonApiDocument = match serde_json::from_str(&string_wallets) {
                    Ok(value)  => value,
                    Err(err) => return Failure((Status::BadRequest, format!("{:?}", err)))
                };

                match Self::from_jsonapi_document(&raw_json) {
                    Ok(wallets) => Success(wallets),
                    Err(err) => return Failure((Status::BadRequest, format!("{:?}", err)))
                }
            }
        }
    };
}
