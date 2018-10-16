use jsonapi::model::*;
use std;
pub trait ResourceAddress:
    JsonApiModel
    + PartialEq
    + std::marker::Sized
    + std::fmt::Debug
    + std::clone::Clone
    + std::fmt::Display
{
}
