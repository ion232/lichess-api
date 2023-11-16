use crate::model::{Domain, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteQuery;

pub type DeleteRequest = Request<DeleteQuery>;

impl DeleteRequest {
    pub fn new(id: &str) -> Self {
        Self {
            domain: Domain::Engine,
            method: http::Method::DELETE,
            path: format!("/api/external-engine/{}", id),
            query: Default::default(),
            body: Default::default(),
        }
    }
}
