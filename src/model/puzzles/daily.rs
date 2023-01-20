use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct Query;
pub type GetRequest = crate::model::Request<Query>;

impl GetRequest {
    pub fn new() -> Self {
        Self {
            method: http::Method::GET,
            path: "/api/puzzle/daily".to_string(),
            query: Default::default(),
            body: Default::default()
        }
    }
}

pub type Puzzle = super::PuzzleJson;
