use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(challenge_id: &str) -> Self {
        let path = format!("/api/challenge/{challenge_id}/show");
        Self::get(path, None, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(s: S) -> Self {
        Self::new(s.as_ref())
    }
}
