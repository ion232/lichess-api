use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(game_id: &str) -> Self {
        let path = format!("/api/board/game/{game_id}/claim-victory");
        Self::post(path, None, None, None)
    }
}

impl<S: AsRef<str>> From<S> for PostRequest {
    fn from(s: S) -> Self {
        Self::new(s.as_ref())
    }
}
