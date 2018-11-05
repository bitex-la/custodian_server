pub use std::collections::HashMap;
pub use jsonapi::api::*;
pub use jsonapi::query::{Query, QueryFields};
use jsonapi::errors::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value, Map};

/* This trait constructs JsonApi Documents from structs and our database,
 * it relies on the JsonApi crate and the database
 */
pub trait ToJsonApi {
    const TYPE : &'static str;

		fn attributes(&self, fields: &QueryFields) -> ResourceAttributes;

		fn relationships(&self, fields: &QueryFields) -> Option<Relationships> {
				None
		}

		fn included(&self, fields: &QueryFields) -> Option<Resources> {
				None
		}

    fn to_jsonapi_document(&self, id: usize) -> JsonApiDocument {
				self.to_jsonapi_document_with_query(id, &Default::default())
    }

    fn to_jsonapi_document_with_query(&self, id: usize, query: &Query)
        -> JsonApiDocument
    {
        let (res, included) = self.to_jsonapi_resource_with_query(id, query);
        JsonApiDocument {
            data: Some(PrimaryData::Single(Box::new(res))),
            included: included,
            ..Default::default()
        }
    }

		fn to_jsonapi_resource(&self, id: usize) -> (Resource, Option<Resources>) {
      self.to_jsonapi_resource_with_query(id, &Default::default())
    }

    fn to_jsonapi_resource_with_query(&self, id: usize, query: &Query)
      -> (Resource, Option<Resources>)
    {
				let resource = Resource{
						_type: Self::TYPE,
						id: id,
						relationships: self.relationships(&query.fields),
						attributes: self.attributes(&query.fields),
						..Default::default()
				};
        (resource, self.included(&query.include))
    }

    fn has_one(typ: &str, id: usize) -> Relationship {
        Relationship{
            data: IdentifierData::Single(
								ResourceIdentifier{ id: id, _type: typ }
						),
            links: None
        }
    }
}
