use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(game_id: String, seconds: u32) -> Self {
        let path = format!("/api/round/{}/add-time/{}", game_id, seconds);
        Self {
            method: http::Method::POST,
            path,
            query: Default::default(),
            body: Default::default(),
        }
    }
}
