use crate::model::{Body, Domain, Request};
use serde::Serialize;

use super::UpdateExternalEngine;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutQuery;

pub type PutRequest = Request<PutQuery, UpdateExternalEngine>;

impl PutRequest {
    pub fn new(id: &str, engine: UpdateExternalEngine) -> Self {
        Self {
            domain: Domain::Engine,
            method: http::Method::PUT,
            path: format!("/api/external-engine/{}", id),
            query: Default::default(),
            body: Body::Json(engine),
        }
    }
}
