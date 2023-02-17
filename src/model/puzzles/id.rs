use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(id: &str) -> Self {
        Self {
            method: http::Method::GET,
            path: format!("/api/puzzle/{}", id),
            query: Default::default(),
            body: Default::default(),
        }
    }
}

pub type Puzzle = super::PuzzleJson;
