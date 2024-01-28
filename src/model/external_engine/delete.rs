use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteQuery;

pub type DeleteRequest = Request<DeleteQuery>;

impl DeleteRequest {
    pub fn new(id: &str) -> Self {
        Self {
            method: http::Method::DELETE,
            path: format!("/api/external-engine/{}", id),
            ..Default::default()
        }
    }
}
