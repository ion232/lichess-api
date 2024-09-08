use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery {
    opponent_token: String,
}

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(challenge_id: String, opponent_token: Option<String>) -> Self {
        let path = format!("/api/challenge/{challenge_id}/cancel");
        let query = opponent_token.map(|t| PostQuery { opponent_token: t });
        Self::post(path, query, None, None)
    }
}

impl<S: Into<String>> From<S> for PostRequest {
    fn from(challenge_id: S) -> Self {
        Self::new(challenge_id.into(), None)
    }
}
