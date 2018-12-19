use std::io::Read;
use jsonapi::model::*;
use rocket::http::Status;
use rocket::{Data, Request, State};
use rocket::Outcome::{ Success, Failure };
use rocket::data;
use rocket::data::{ FromData, Transform, Transformed };
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

impl<'a, T> FromData<'a> for Mapped<T>
    where T: FromJsonApi
{
    type Error = String;
    type Owned = String;
    type Borrowed = str;

    fn transform(_: &Request, data: Data) -> Transform<data::Outcome<Self::Owned, Self::Error>> {
        let mut stream = data.open();
        let mut string = String::new();
        let outcome = match stream.read_to_string(&mut string) {
            Ok(_) => Success(string),
            Err(e) => Failure((Status::InternalServerError, e.to_string()))
        };

        Transform::Borrowed(outcome)
    }
    
    fn from_data(request: &Request, outcome: Transformed<'a, Self>) -> data::Outcome<Self, Self::Error> {
        let string_data = outcome.borrowed()?;

        let doc: JsonApiDocument = match ::serde_json::from_str(&string_data) {
            Ok(value) => value,
            Err(err) => {
                println!("Not a jsonapi document {:#?} {:#?}", &string_data, &err);
                return Failure((Status::BadRequest, "Not a json_api document".into()));
            }
        };

        let state: State<ServerState> = match request.guard::<State<ServerState>>() {
            Success(value) => value,
            _ => {
                return Failure((Status::BadRequest, "Can't access db".into()));
            }
        };

        let db = state.database_lock();

        match T::from_json_api_document(doc.clone(), db.clone()) {
            Ok(item) => Success(Mapped(item)),
            Err(err) => {
                println!("Cannot parse from jsonapi document {:#?}, {:#?}", doc, &err);
                Failure((Status::BadRequest, err))
            }
        }
    }
}
