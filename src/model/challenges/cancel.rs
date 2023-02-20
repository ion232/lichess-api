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
        let path = format!("/api/challenge/{}/cancel", challenge_id);
        Self {
            method: http::Method::POST,
            path,
            query: opponent_token.map(|t| PostQuery { opponent_token: t }),
            body: Default::default(),
        }
    }
}
