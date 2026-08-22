use crate::model::Request;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(id: &str) -> Self {
        let path = format!("/api/racer/{id}");
        Self::get(path, None, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(s: S) -> Self {
        Self::new(s.as_ref())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceResults {
    pub id: String,
    pub owner: String,
    pub players: Vec<RacePlayer>,
    pub puzzles: Vec<RacePuzzle>,
    pub finishes_at: u64,
    pub starts_at: u64,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RacePlayer {
    pub name: String,
    pub score: i32,
    pub id: Option<String>,
    pub flair: Option<String>,
    /// Deprecated: use `patron_color` instead.
    pub patron: Option<bool>,
    pub patron_color: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RacePuzzle {
    pub id: String,
    pub fen: String,
    pub line: String,
    pub rating: i32,
}
