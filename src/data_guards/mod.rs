use std::io::Read;
use jsonapi::model::JsonApiDocument;
use models::database::Database;
use rocket::data::FromData;
use rocket::http::Status;
use rocket::{Data, Outcome, Request, State};
use rocket::data;

pub trait FromJsonApiDocument {
    fn from_json_api_document(doc: JsonApiDocument, db: Database) -> Result<Self, String>;
}

impl<T: FromJsonApiDocument> FromData for T {
    type Error = String;

    fn from_data(request: &Request, data: Data) -> data::Outcome<T, String> {
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

        let db: State<Database> = match request.guard::<State<Database>>() {
            Outcome::Success(state) => state,
            Outcome::Failure(err) => {
                println!("Can't obtain database {:#?}", &err);
                return Outcome::Failure((Status::InternalServerError, "Can't obtain database".into()));
            }
        };

        match T::from_json_api_document(doc, db.clone()) {
            Ok(item) => Outcome::Success(item),
            Err(err) => {
                println!("Cannot parse from jsonapi document {:#?}, {:#?}", &doc, &err);
                Outcome::Failure((Status::BadRequest, err))
            }
        }
    }
}
