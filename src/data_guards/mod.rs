trait FromJsonApiDocument {
    fn from_json_api_document(doc: JsonApiDocument, db: Database) -> Result<Self>;
}

impl<T: FromJsonApiDocument> FromData for T {
    type Error = String;

    fn from_json_api_data(request: &Request, data: Data)
      -> Outcome<T, String>
    {
        let mut string_data = String::new();
        if let Err(e) = data.open().read_to_string(&mut string_data) {
            return Outcome::Failure((
                Status::InternalServerError,
                format!("{:#?}", e),
            ));
        }

        let doc: JsonApiDocument = match ::serde_json::from_str(&string_data) {
            Ok(value) => value,
            Err(err) => {
                println!("Not a jsonapi document {:#?} {:#?}", &string_data, &err);
                return Outcome::Failure((
                    Status::BadRequest,
                    "Not a json_api document".into(),
                ));
            }
        };

        let db : Database = request.guard::<State<Database>>()?;
        match T::from_json_api_document(doc, db) {
            Ok(item) => Outcome::Success(item),
            Err(err) => {
                println!(
                    "Cannot parse from jsonapi document {:#?}, {:#?}",
                    &doc, err
                );
                Outcome::Failure((
                    Status::BadRequest,
                    "Cannot parse resource from document".into(),
                ));
            }
        }
    }

}

