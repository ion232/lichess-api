use crate::model::{Body, Request};
use serde::Serialize;

use super::UpdateExternalEngine;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutQuery;

pub type PutRequest = Request<PutQuery, UpdateExternalEngine>;

impl PutRequest {
    pub fn new(id: &str, engine: UpdateExternalEngine) -> Self {
        let path = format!("/api/external-engine/{id}");
        Self::put(path, None, Body::Json(engine), None)
    }
}
