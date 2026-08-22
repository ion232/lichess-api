use crate::model::Request;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    pub v: Option<bool>,
}

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(game_id: &str, query: PostQuery) -> Self {
        let path = format!("/bookmark/{game_id}");
        Self::post(path, query, None, None)
    }
}
