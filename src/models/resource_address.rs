use std;
use jsonapi::model::*;
pub trait ResourceAddress : JsonApiModel + PartialEq + std::marker::Sized + std::fmt::Debug + std::clone::Clone {}
