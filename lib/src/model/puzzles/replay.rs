use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(days: u32, theme: &str) -> Self {
        Self::get(format!("/api/puzzle/replay/{days}/{theme}"), None, None)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleReplay {
    pub replay: ReplayData,
    pub angle: AngleData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayData {
    pub days: u32,
    pub theme: String,
    pub nb: u32,
    pub remaining: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AngleData {
    pub key: String,
    pub name: String,
    pub desc: String,
}

pub type Replay = PuzzleReplay;