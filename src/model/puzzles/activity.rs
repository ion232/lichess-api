use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub max: u32,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(max_rounds: Option<u32>) -> Self {
        Self {
            method: http::Method::GET,
            path: "/api/puzzle/activity".to_string(),
            query: max_rounds.map(|max| GetQuery { max }),
            body: Default::default(),
        }
    }
}

pub type Round = PuzzleRoundJson;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PuzzleRoundJson {
    pub id: String,
    pub date: u64,
    pub win: bool,
    pub puzzle_rating: u32,
}
