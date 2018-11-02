/* This trait helps us parsing requests that contain one resource that may
 * have relationships.
 * It tries to fiend the relationships in our database, and to fetch all
 * the required fields from the received resource.
 * Once a struct is parsed from the request, it's ready to be inserted in the db
 */
pub trait FromJsonApi: Sized {
    const TYPE : &'static str;

    fn attribute<A>(resource: &Resource, attr_name: &str) -> Result<A, String>
        where A: for<'de> serde::Deserialize<'de>
    {
        let value = resource.attributes.get(attr_name)
          .ok_or_else(|| format!("Attribute {} not found", &attr_name))?;
        serde_json::from_value(value.clone())
          .map_err(|_| format!("Invalid type for {}", &attr_name))
    }

    fn has_one_id(resource: &Resource, name: &str) -> Result<usize, String> {
        let all = resource.relationships.as_ref()
            .ok_or_else(|| format!("No relationships at all"))?
            .get(name)
            .ok_or_else(|| format!("No relationship called {}", name))?;

        let identifier = match all {
            Relationship{ data: IdentifierData::Single(it), .. } => it,
            _ => return Err(format!("Could not parse a single {}", name))
        };

        identifier.id.parse::<usize>()
            .map_err(|_| format!("Could not parse {} id", name))
    }

    fn from_json_api_document(doc: JsonApiDocument, db: Database) -> Result<Self, String> {
        let resource = match doc.data {
            Some(PrimaryData::Single(res)) => res,
            _ => return Err(format!("Invalid document data"))
        };

        if resource._type != Self::TYPE {
            return Err("Type was wrong".into());
        }

        Self::from_json_api_resource(*resource, db)
    }

    fn from_json_api_resource(doc: Resource, db: Database) -> Result<Self, String>;
}

