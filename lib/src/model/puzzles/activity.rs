use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub max: u32,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(max_rounds: Option<u32>) -> Self {
        let query = max_rounds.map(|max| GetQuery { max });
        Self::get("/api/puzzle/activity", query, None)
    }
}

impl Default for GetRequest {
    fn default() -> Self {
        Self::new(None)
    }
}

impl From<u32> for GetRequest {
    fn from(max: u32) -> Self {
        Self::new(Some(max))
    }
}

pub type Round = PuzzleRoundJson;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PuzzleRoundJson {
    pub date: u64,
    pub win: bool,
    pub puzzle: Puzzle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Puzzle {
    pub id: String,
    pub fen: String,
    pub plays: i32,
    pub rating: i32,
    pub solution: Vec<String>,
    pub themes: Vec<String>,
}
