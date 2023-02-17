use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(game_id: &str) -> Self {
        let path = format!("/api/board/game/{}/claim-victory", game_id);
        Self {
            method: http::Method::POST,
            path,
            query: Default::default(),
            body: Default::default(),
        }
    }
}
