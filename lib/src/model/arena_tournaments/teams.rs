use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(id: &str) -> Self {
        Self::get(format!("/api/tournament/{id}/teams"), None, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(id: S) -> Self {
        Self::new(id.as_ref())
    }
}
