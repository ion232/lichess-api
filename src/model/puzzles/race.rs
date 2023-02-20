use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, serde::Serialize)]
pub struct PostQuery;

pub type PostRequest = crate::model::Request<PostQuery>;

impl PostRequest {
    pub fn new() -> Self {
        Self {
            method: http::Method::POST,
            path: "/api/racer".to_string(),
            query: Default::default(),
            body: Default::default(),
        }
    }
}

pub type Race = PuzzleRaceJson;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleRaceJson {
    id: String,
    url: String,
}
