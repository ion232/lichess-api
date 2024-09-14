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

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Follows {
    #[serde(rename = "in")]
    pub gained: Option<Followers>,

    #[serde(rename = "out")]
    pub lost: Option<Followers>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorrespondenceOpponent {
    pub user: Option<String>,
    pub rating: Option<u32>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorrespondenceGame {
    pub id: String,
    pub color: Color,
    pub url: String,
    pub opponent: CorrespondenceOpponent,
    pub variant: Option<String>,
    pub speed: Option<String>,
    pub perf: Option<String>,
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
pub struct TournamentId {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub tournament: TournamentId,

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

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub interval: Range<u64>,
    pub games: Option<HashMap<PerfType, Score>>,
    pub puzzles: Option<Puzzles>,
    pub follows: Option<Follows>,
    pub correspondence_moves: Option<CorrespondenceMoves>,
    pub correspondence_ends: Option<CorrespondenceEnds>,
    pub tournaments: Option<Tournaments>,
    pub stream: Option<bool>,
    pub posts: Option<Vec<Topic>>,
    pub practice: Option<Vec<Practice>>,
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
