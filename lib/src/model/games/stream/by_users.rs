use crate::model::Body;
use serde::Serialize;
use std::borrow::Borrow;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    pub with_current_games: bool,
}

pub type PostRequest = crate::model::Request<PostQuery, Vec<String>>;

impl PostRequest {
    pub fn new<Id: Borrow<str>, Ids: AsRef<[Id]>>(user_ids: Ids, with_current_games: bool) -> Self {
        let body = Body::PlainText(user_ids.as_ref().join(","));
        let query = PostQuery { with_current_games };
        Self::post("/api/stream/games-by-users", query, body, None)
    }
}
