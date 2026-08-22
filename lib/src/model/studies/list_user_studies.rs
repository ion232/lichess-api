use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str) -> Self {
        Self::get(format!("/api/study/by/{username}"), None, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(username: S) -> Self {
        Self::new(username.as_ref())
    }
}
