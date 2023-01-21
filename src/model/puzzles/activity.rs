use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct Query {
    pub max: u32
}

pub type GetRequest = crate::model::Request<Query>;

impl GetRequest {
    pub fn new(max_rounds: Option<u32>) -> Self {
        Self {
            method: http::Method::GET,
            path: "/api/puzzle/activity".to_string(),
            query: max_rounds.map(|max| Query{max}),
            body: Default::default()
        }
    }
}

pub type Round = PuzzleRoundJson;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleRoundJson {
    pub id: String,
    pub date: u64,
    pub win: bool,
    #[serde(rename = "puzzleRating")] 
    pub puzzle_rating: u32
}
