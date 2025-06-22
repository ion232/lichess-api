use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub angle: Option<String>,
    pub difficulty: Option<Difficulty>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easiest,
    Easier,
    Normal,
    Harder,
    Hardest,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(angle: Option<String>, difficulty: Option<Difficulty>) -> Self {
        let query = GetQuery { angle, difficulty };
        Self::get("/api/puzzle/next", query, None)
    }
}

impl Default for GetRequest {
    fn default() -> Self {
        Self::new(None, None)
    }
}

pub type Puzzle = super::PuzzleAndGame;