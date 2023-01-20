use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct Query { max: u32 }
pub type GetRequest =  crate::model::Request<Query>;

impl GetRequest {
    pub fn new(query: Option<Query>) -> Self {
        Self {
            method: http::Method::GET,
            path: "/api/puzzle/activity".to_string(),
            query,
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
