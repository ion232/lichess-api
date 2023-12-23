use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub days: u32,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str, days: Option<u32>) -> Self {
        let path = format!("/api/storm/dashboard/{}", username);
        Self {
            path,
            query: days.map(|x| GetQuery { days: x }),
            ..Default::default()
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
