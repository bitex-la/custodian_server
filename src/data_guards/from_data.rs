#[macro_export]
macro_rules! from_data {
    ($type:ty) => {
        impl ::rocket::data::FromData for $type {
            type Error = String;

            fn from_data(
                _: &::rocket::Request,
                data: ::rocket::Data,
            ) -> ::rocket::data::Outcome<Self, String> {
                let mut string_data = String::new();
                if let Err(e) = data.open().read_to_string(&mut string_data) {
                    return ::rocket::Outcome::Failure((
                        ::rocket::http::Status::InternalServerError,
                        format!("{:#?}", e),
                    ));
                }

                let raw_json: JsonApiDocument = match ::serde_json::from_str(&string_data) {
                    Ok(value) => value,
                    Err(err) => {
                        println!("Not a jsonapi document {:#?}", &string_data);
                        return ::rocket::Outcome::Failure((
                            ::rocket::http::Status::BadRequest,
                            format!("Not a json_api document {:#?}", err),
                        ));
                    }
                };

                match Self::from_jsonapi_document(&raw_json) {
                    Ok(data) => ::rocket::Outcome::Success(data),
                    Err(err) => {
                        println!(
                            "Cannot parse from jsonapi document {:#?}, {:#?}",
                            &raw_json, err
                        );
                        return ::rocket::Outcome::Failure((
                            ::rocket::http::Status::BadRequest,
                            format!("Cannot parse resource from document {:#?}", err),
                        ));
                    }
                }
            }
        }
    };
}
