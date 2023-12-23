use super::Base;
use crate::model::Body;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    #[serde(flatten)]
    pub base: Base,
}

pub type PostRequest = crate::model::Request<PostQuery, String>;

impl PostRequest {
    pub fn new(game_ids: Vec<String>, query: PostQuery) -> Self {
        Self {
            method: http::Method::POST,
            path: "/api/games/export/_ids".to_string(),
            query: Some(query),
            body: Body::PlainText(game_ids.join(",")),
            ..Default::default()
        }
    }
}
