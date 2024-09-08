use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(challenge_id: &str) -> Self {
        let path = format!("/api/challenge/{challenge_id}/accept");
        Self::post(path, None, None, None)
    }
}

impl<S: AsRef<str>> From<S> for PostRequest {
    fn from(s: S) -> Self {
        Self::new(s.as_ref())
    }
}
