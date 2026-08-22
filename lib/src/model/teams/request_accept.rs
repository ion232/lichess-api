use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(team_id: &str, user_id: &str) -> Self {
        Self::post(
            format!("/api/team/{team_id}/request/{user_id}/accept"),
            None,
            None,
            None,
        )
    }
}

impl<S: AsRef<str>> From<(S, S)> for PostRequest {
    fn from((team_id, user_id): (S, S)) -> Self {
        Self::new(team_id.as_ref(), user_id.as_ref())
    }
}
