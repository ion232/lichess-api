use crate::model::Body;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    pub with_current_games: bool,
}

pub type PostRequest = crate::model::Request<PostQuery, Vec<String>>;

impl PostRequest {
    pub fn new(user_ids: Vec<String>, with_current_games: bool) -> Self {
        Self {
            method: http::Method::POST,
            path: "/api/stream/games-by-users".to_string(),
            query: Some(PostQuery { with_current_games }),
            body: Body::PlainText(user_ids.join(",")),
        }
    }
}
