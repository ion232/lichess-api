use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self {
            path: "/api/puzzle/daily".to_string(),
            ..Default::default()
        }
    }
}

pub type Puzzle = super::PuzzleAndGame;
