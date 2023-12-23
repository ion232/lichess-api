use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(challenge_id: String) -> Self {
        let path = format!("/api/challenge/{}/accept", challenge_id);
        Self {
            method: http::Method::POST,
            path,
            ..Default::default()
        }
    }
}
