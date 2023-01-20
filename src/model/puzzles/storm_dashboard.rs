use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct Query {
    pub days: u32
}

pub type GetRequest = crate::model::Request<Query>;

impl GetRequest {
    pub fn new(username: &str, query: Option<Query>) -> Self {
        let path = format!("/api/storm/dashboard/{}", username);
        Self {
            method: http::Method::GET,
            path,
            query,
            body: Default::default()
        }
    }
}

pub type Dashboard = StormDashboardJson;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StormDashboardJson {
    high: High,
    days: Vec<Day>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct High {
    #[serde(rename = "allTime")]
    all_time: i64,
    day: i64,
    month: i64,
    week: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Day {
    #[serde(rename = "_id")]
    id: String,
    combo: i64,
    errors: i64,
    highest: i64,
    moves: i64,
    runs: i64,
    score: i64,
    time: i64,
}
