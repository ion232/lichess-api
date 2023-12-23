use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub max: u32,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(max_rounds: Option<u32>) -> Self {
        Self {
            path: "/api/puzzle/activity".to_string(),
            query: max_rounds.map(|max| GetQuery { max }),
            ..Default::default()
        }
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
