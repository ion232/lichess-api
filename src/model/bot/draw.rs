use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(game_id: &str, accept: bool) -> Self {
        let accept = if accept { "yes" } else { "no" };
        let path = format!("/api/bot/game/{}/draw/{}", game_id, accept);
        Self {
            method: http::Method::POST,
            path,
            ..Default::default()
        }
    }
}
