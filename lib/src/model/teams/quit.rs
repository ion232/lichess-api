use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(team_id: &str) -> Self {
        Self::post(format!("/team/{team_id}/quit"), None, None, None)
    }
}

impl<S: AsRef<str>> From<S> for PostRequest {
    fn from(team_id: S) -> Self {
        Self::new(team_id.as_ref())
    }
}
