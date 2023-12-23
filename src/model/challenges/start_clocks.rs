use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    pub token1: String,
    pub token2: String,
}

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(game_id: String, token1: String, token2: String) -> Self {
        let path = format!("/api/challenge/{}/start-clocks", game_id);
        Self {
            method: http::Method::POST,
            path,
            query: Some(PostQuery { token1, token2 }),
            ..Default::default()
        }
    }
}
