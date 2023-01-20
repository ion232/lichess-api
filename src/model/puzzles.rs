use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Clone, Debug, Serialize)]
pub struct Query;
pub type GetRequest = crate::model::Request<Query>;

impl GetRequest {
    pub fn new(id: &str) -> Self {
        Self {
            method: http::Method::GET,
            path: format!("/api/puzzle/{}", id),
            query: Default::default(),
            body: Default::default()
        }
    }
}

pub mod daily {
    #[derive(Default, Clone, Debug, serde::Serialize)]
    pub struct Query;
    pub type GetRequest = crate::model::Request<Query>;

    impl GetRequest {
        pub fn new() -> Self {
            Self {
                method: http::Method::GET,
                path: "/api/puzzle/daily".to_string(),
                query: Default::default(),
                body: Default::default()
            }
        }
    }

    pub type Puzzle = super::PuzzleJson;
}

pub mod activity {
    #[derive(Default, Clone, Debug, serde::Serialize)]
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

    pub type Round = super::PuzzleRoundJson;
}

pub mod dashboard {
    #[derive(Default, Clone, Debug, serde::Serialize)]
    pub struct Query { max: u32 }
    pub type GetRequest = crate::model::Request<Query>;

    impl GetRequest {
        pub fn new(max: u32, query: Option<Query>) -> Self {
            Self {
                method: http::Method::GET,
                path: format!("/api/puzzle/dashboard/{}", max),
                query,
                body: Default::default()
            }
        }
    }

    pub type Dashboard = super::PuzzleDashboardJson;
}

pub mod storm_dashboard {
    #[derive(Default, Clone, Debug, serde::Serialize)]
    pub struct Query { days: u32 }
    pub type GetRequest = crate::model::Request<Query>;

    impl GetRequest {
        fn new(username: &str, query: Option<Query>) -> Self {
            let path = format!("/api/storm/dashboard/{}", username);
            Self {
                method: http::Method::GET,
                path,
                query,
                body: Default::default()
            }
        }
    }

    pub type Dashboard = super::StormDashboardJson;
}

pub mod race {
    #[derive(Default, Clone, Debug, serde::Serialize)]
    pub struct Query;
    pub type PostRequest =  crate::model::Request<Query>;

    impl PostRequest {
        fn new() -> Self {
            Self {
                method: http::Method::GET,
                path: "/api/racer".to_string(),
                query: Default::default(),
                body: Default::default()
            }
        }
    }

    pub type Race = super::PuzzleRaceJson;
}

pub type PuzzleResponse = PuzzleJson;

// Underlying structs.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleDashboardJson {
    days: i64,
    global: Results,
    themes: HashMap<String, Theme>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleRaceJson {
    id: String,
    url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleRoundJson {
    pub id: String,
    pub date: u64,
    pub win: bool,
    #[serde(rename = "puzzleRating")] 
    pub puzzle_rating: u32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleJson {
    pub game: Game,
    pub puzzle: Puzzle
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StormDashboardJson {
    high: High,
    days: Vec<Day>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct High {
    #[serde(rename = "allTime")]
    all_time: i64,
    day: i64,
    month: i64,
    week: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    results: Results,
    theme: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Results {
    #[serde(rename = "firstWins")]
    first_wins: i64,
    nb: i64,
    performance: i64,
    #[serde(rename = "puzzleRatingAvg")]
    puzzle_rating_avg: i64,
    #[serde(rename = "replayWins")]
    replay_wins: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Perf {
    pub key: String,
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub color: String,
    pub name: String,
    #[serde(rename = "userId")] 
    pub user_id: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Game {
    pub clock: String,
    pub id: String,
    pub perf: Perf,
    pub pgn: String,
    pub players: Vec<Player>,
    pub rated: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Puzzle {
    pub id: String,
    #[serde(rename = "initialPly")] 
    pub initial_ply: i32,
    pub plays: i32,
    pub rating: i32,
    pub solution: Vec<String>,
    pub themes: Vec<String>
}
