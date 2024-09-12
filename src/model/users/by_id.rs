use crate::model::{Body, Request};
use serde::Serialize;
use std::borrow::Borrow;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;
pub type PostRequest = Request<PostQuery, String>;

impl PostRequest {
    pub fn new<Ids: AsRef<[Id]>, Id: Borrow<str>>(ids: Ids) -> Self {
        let ids = ids.as_ref().join(",");
        Self::post("/api/users", None, Body::PlainText(ids), None)
    }
}

impl<Id: Borrow<str>> From<&[Id]> for PostRequest {
    fn from(ids: &[Id]) -> Self {
        Self::new(ids)
    }
}

impl<const N: usize, Id: Borrow<str>> From<&[Id; N]> for PostRequest {
    fn from(ids: &[Id; N]) -> Self {
        Self::new(ids)
    }
}

impl<const N: usize, Id: Borrow<str>> From<[Id; N]> for PostRequest {
    fn from(ids: [Id; N]) -> Self {
        Self::new(ids)
    }
}

impl<Id: Borrow<str>> From<&Vec<Id>> for PostRequest {
    fn from(ids: &Vec<Id>) -> Self {
        Self::new(ids)
    }
}

impl<Id: Borrow<str>> From<Vec<Id>> for PostRequest {
    fn from(ids: Vec<Id>) -> Self {
        Self::new(ids)
    }
}
