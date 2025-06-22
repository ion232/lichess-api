use crate::model::Body;
use serde::Serialize;
use std::borrow::Borrow;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = crate::model::Request<PostQuery, Vec<String>>;

impl PostRequest {
    pub fn new<Id: Borrow<str>, Ids: AsRef<[Id]>>(stream_id: &str, game_ids: Ids) -> Self {
        let path = format!("/api/stream/games/{stream_id}");
        let body = Body::PlainText(game_ids.as_ref().join(","));
        Self::post(path, None, body, None)
    }
}
