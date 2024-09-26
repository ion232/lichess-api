use crate::model::{puzzles, PerfType, Request, Title};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Glicko {
    pub rating: f64,
    pub deviation: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisional: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Perf {
    pub glicko: Glicko,

    #[serde(rename = "nb")]
    pub rated_games: u32,
    pub progress: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub at: String,
    pub game_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingExtreme {
    #[serde(rename = "int")]
    pub rating: u32,

    #[serde(flatten)]
    pub game: Game,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserId {
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WinExtremeGame {
    #[serde(rename = "opRating")]
    pub opponent_rating: u32,

    #[serde(rename = "opId")]
    pub opponent: UserId,

    #[serde(flatten)]
    pub game: Game,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WinExtremes {
    pub results: Vec<WinExtremeGame>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub all: u32,
    pub rated: u32,
    pub win: u32,
    pub loss: u32,
    pub draw: u32,
    pub tour: u32,
    pub berserk: u32,
    pub op_avg: f64,
    pub seconds: u32,
    pub disconnects: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Streak {
    Range { v: u32, from: Game, to: Game },
    Empty { v: u32 },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreakSet {
    #[serde(rename = "cur")]
    pub current: Streak,
    pub max: Streak,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResultStreak {
    pub win: StreakSet,
    pub loss: StreakSet,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayStreak {
    #[serde(rename = "nb")]
    pub game_count: StreakSet,
    pub time: StreakSet,
    pub last_date: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub perf_type: puzzles::Perf,
    pub id: String,
    pub highest: RatingExtreme,
    pub lowest: RatingExtreme,
    pub best_wins: WinExtremes,
    pub worst_losses: WinExtremes,
    pub count: Count,
    pub result_streak: ResultStreak,
    pub play_streak: PlayStreak,
    pub user_id: UserId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Performance {
    pub user: User,
    pub perf: Perf,
    pub rank: Option<u32>,
    pub percentile: f64,
    pub stat: Stat,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str, perf: PerfType) -> Self {
        Self::get(format!("/api/user/{username}/perf/{perf}"), None, None)
    }
}
