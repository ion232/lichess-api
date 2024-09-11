use crate::model::{Color, PerfType, Request};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Range};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Interval {
    pub from: u64,
    pub to: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RatingProgress {
    pub before: u32,
    pub after: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Score {
    pub win: u32,
    pub loss: u32,
    pub draw: u32,

    #[serde(rename = "rp")]
    pub rating_progress: RatingProgress,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Puzzles {
    pub score: Score,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Followers {
    pub ids: Vec<String>,

    #[serde(rename = "nb", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Follows {
    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gained: Option<Followers>,

    #[serde(rename = "out")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lost: Option<Followers>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorrsepondenceOpponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde_with::skip_serializing_none]
pub struct CorrespondenceGame {
    pub id: String,
    pub color: Color,
    pub url: String,
    pub opponent: CorrsepondenceOpponent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorrespondenceMoves {
    #[serde(rename = "nb")]
    pub move_count: u32,
    pub games: Vec<CorrespondenceGame>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorrespondenceEnds {
    pub score: Score,
    pub games: Vec<CorrespondenceGame>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TournamentID {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub tournament: TournamentID,

    #[serde(rename = "nbGames")]
    pub game_count: u32,
    pub score: u32,
    pub rank: u32,
    pub rank_percent: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tournaments {
    #[serde(rename = "nb")]
    pub count: u32,
    pub best: Vec<Tournament>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    pub url: String,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    pub topic_url: String,
    pub topic_name: String,
    pub posts: Vec<Post>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Practice {
    pub url: String,
    pub name: String,

    #[serde(rename = "nbPositions")]
    pub positions_count: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Team {
    pub url: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub interval: Range<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub games: Option<HashMap<PerfType, Score>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub puzzles: Option<Puzzles>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub follows: Option<Follows>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondence_moves: Option<CorrespondenceMoves>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondence_ends: Option<CorrespondenceEnds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tournaments: Option<Tournaments>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<Topic>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice: Option<Vec<Practice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<Team>>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str, perf: PerfType) -> Self {
        Self::get(format!("/api/user/{username}/perf/{perf}"), None, None)
    }
}
