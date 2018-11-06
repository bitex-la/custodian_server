pub use jsonapi::api::*;
pub use jsonapi::query::{Query, QueryFields};
pub use jsonapi::model::JsonApiModel;
use std;
use tiny_ram_db::Record;

/* This trait constructs JsonApi Documents from structs and our database,
 * it relies on the JsonApi crate and the database
 */
pub trait ToJsonApi {
    const TYPE: &'static str;

    fn id(&self) -> usize { 0 }

    fn attributes(&self, fields: &QueryFields) -> ResourceAttributes;

    fn relationships(&self, _fields: &QueryFields) -> Option<Relationships> {
        None
    }

    fn included(&self, _fields: &Vec<String>) -> Option<Resources> {
        None
    }

    fn to_jsonapi_document(&self, id: usize) -> JsonApiDocument {
        self.to_jsonapi_document_with_query(id, &Default::default())
    }

    fn to_jsonapi_document_with_query(&self, id: usize, query: &Query) -> JsonApiDocument {
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

    fn to_jsonapi_resource_with_query(
        &self,
        id: usize,
        query: &Query,
    ) -> (Resource, Option<Resources>) {
        let resource = Resource {
            _type: Self::TYPE.to_string(),
            id: Some(id.to_string()),
            relationships: self.relationships(&query.fields),
            attributes: self.attributes(&query.fields),
            ..Default::default()
        };
        (resource, self.included(&query.include.as_ref().unwrap_or(&vec![])))
    }

    fn has_one(typ: &str, id: usize) -> Relationship {
        Relationship {
            data: IdentifierData::Single(ResourceIdentifier {
                id: id.to_string(),
                _type: typ.to_string(),
            }),
            links: None,
        }
    }

    fn collection_to_jsonapi_document<Collection, T>(objects: Collection) -> JsonApiDocument 
        where
            T: ToJsonApi,
            Collection: std::iter::IntoIterator<Item = T>
    {
        Self::collection_to_jsonapi_document_with_query(objects, &Default::default())
    }

    fn collection_to_jsonapi_document_with_query<Collection, T>(
        objects: Collection,
        query: &Query,
    ) -> JsonApiDocument
        where
            T: ToJsonApi,
            Collection: std::iter::IntoIterator<Item = T>
    {
        let (resources, included) = Self::collection_to_jsonapi_resources(objects, query);
        JsonApiDocument {
            data: Some(PrimaryData::Multiple(resources)),
            included: included,
            ..Default::default()
        }
    }

    fn collection_to_jsonapi_resources<Collection, T>(
        objects: Collection,
        query: &Query,
    ) -> (Resources, Option<Resources>) 
        where
            T: ToJsonApi,
            Collection: std::iter::IntoIterator<Item = T>
    {
        let mut included = vec![];
        let resources = objects
            .into_iter()
            .map(|obj| {
                let (res, mut opt_incl) = obj.to_jsonapi_resource_with_query(obj.id(), query);
                if let Some(ref mut incl) = opt_incl {
                    included.append(incl);
                }
                res
            })
            .collect::<Vec<_>>();
        let opt_included = if included.is_empty() {
            None
        } else {
            Some(included)
        };
        (resources, opt_included)
    }
}

impl<T> ToJsonApi for Record<T>
    where T: ToJsonApi
{
    const TYPE: &'static str = T::TYPE;

    fn id(&self) -> usize { self.id }

    fn attributes(&self, fields: &QueryFields) -> ResourceAttributes {
        self.data.attributes(fields)
    }

    fn relationships(&self, fields: &QueryFields) -> Option<Relationships> {
        self.data.relationships(fields)
    }

    fn included(&self, fields: &Vec<String>) -> Option<Resources> {
        self.data.included(fields)
    }
}
