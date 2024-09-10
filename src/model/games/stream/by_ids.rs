use crate::model::Body;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = crate::model::Request<PostQuery, Vec<String>>;

impl PostRequest {
    pub fn new(stream_id: &str, game_ids: Vec<String>) -> Self {
        let path = format!("/api/stream/games/{stream_id}");
        Self::post(path, None, Body::PlainText(game_ids.join(",")), None)
    }
}
