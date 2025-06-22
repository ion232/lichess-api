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

pub type Race = PuzzleRaceJson;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleRaceJson {
    id: String,
    url: String,
}
