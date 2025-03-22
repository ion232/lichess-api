use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteQuery;

pub type DeleteRequest = Request<DeleteQuery>;

impl DeleteRequest {
    pub fn new(id: &str) -> Self {
        Self::delete(format!("/api/external-engine/{id}"), None, None, None)
    }
}

impl<S: AsRef<str>> From<S> for DeleteRequest {
    fn from(s: S) -> Self {
        Self::new(s.as_ref())
    }
}
