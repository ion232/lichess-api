use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, serde::Serialize)]
pub struct PostQuery;

pub type PostRequest = crate::model::Request<PostQuery>;

impl PostRequest {
    pub fn new() -> Self {
        Self::post("/api/racer", None, None, None)
    }
}

impl Default for PostRequest {
    fn default() -> Self {
        Self::new()
    }
}

pub type Race = PuzzleRacer;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleRacer {
    id: String,
    url: String,
}
