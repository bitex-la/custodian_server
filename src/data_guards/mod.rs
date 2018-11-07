use std::io::Read;
use jsonapi::model::*;
use rocket::http::Status;
use rocket::{Data, Outcome, Request, State};
use rocket::data;
use rocket::data::FromData;
use std::ops::Deref;
use serializers::FromJsonApi;
use server_state::ServerState;

pub struct Mapped<T>(pub T);

impl<T> Deref for Mapped<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> FromData for Mapped<T>
    where T: FromJsonApi
{
    type Error = String;
    
    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, String> {

        let mut string_data = String::new();
        if let Err(e) = data.open().read_to_string(&mut string_data) {
            return Outcome::Failure((Status::InternalServerError, format!("{:#?}", e)));
        }

        let doc: JsonApiDocument = match ::serde_json::from_str(&string_data) {
            Ok(value) => value,
            Err(err) => {
                println!("Not a jsonapi document {:#?} {:#?}", &string_data, &err);
                return Outcome::Failure((Status::BadRequest, "Not a json_api document".into()));
            }
        };

        let state: State<ServerState> = match request.guard::<State<ServerState>>() {
            Outcome::Success(value) => value,
            _ => {
                return Outcome::Failure((Status::BadRequest, "Can't access db".into()));
            }
        };

        let db = state.database_lock();

        match T::from_json_api_document(doc.clone(), db.clone()) {
            Ok(item) => Outcome::Success(Mapped(item)),
            Err(err) => {
                println!("Cannot parse from jsonapi document {:#?}, {:#?}", doc, &err);
                Outcome::Failure((Status::BadRequest, err))
            }
        }
    }
}
