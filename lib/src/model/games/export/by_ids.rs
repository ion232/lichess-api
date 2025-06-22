use crate::model::{games::export::Base, Body};
use serde::Serialize;
use std::borrow::Borrow;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    #[serde(flatten)]
    pub base: Base,
}

pub type PostRequest = crate::model::Request<PostQuery, String>;

impl PostRequest {
    pub fn new<Id: Borrow<str>, Ids: AsRef<[Id]>>(game_ids: Ids, query: PostQuery) -> Self {
        let body = Body::PlainText(game_ids.as_ref().join(","));
        Self::post("/api/games/export/_ids", query, body, None)
    }
}
