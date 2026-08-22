use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct DeleteQuery;

pub type DeleteRequest = Request<DeleteQuery>;

impl DeleteRequest {
    pub fn new(id: &str) -> Self {
        Self::delete(format!("/api/bulk-pairing/{id}"), None, None, None)
    }
}

impl<S: AsRef<str>> From<S> for DeleteRequest {
    fn from(id: S) -> Self {
        Self::new(id.as_ref())
    }
}
