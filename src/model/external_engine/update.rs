use crate::model::{Body, Request};
use serde::Serialize;

use super::UpdateExternalEngine;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutQuery;

pub type PutRequest = Request<PutQuery, UpdateExternalEngine>;

impl PutRequest {
    pub fn new(id: &str, engine: UpdateExternalEngine) -> Self {
        Self {
            method: http::Method::PUT,
            path: format!("/api/external-engine/{}", id),
            body: Body::Json(engine),
            ..Default::default()
        }
    }
}
