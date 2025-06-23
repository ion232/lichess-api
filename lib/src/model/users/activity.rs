use crate::model::{Color, Request, VariantKey};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str) -> Self {
        Self::get(format!("/api/user/{username}/activity"), None, None)
    }
}

// Main UserActivity struct matching OpenAPI spec
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserActivity {
    pub interval: ActivityInterval,
    pub games: Option<HashMap<String, UserActivityScore>>,
    pub puzzles: Option<PuzzleActivity>,
    pub storm: Option<PuzzleModePerf>,
    pub racer: Option<PuzzleModePerf>,
    pub streak: Option<PuzzleModePerf>,
    pub tournaments: Option<TournamentActivity>,
    pub practice: Option<Vec<PracticeActivity>>,
    pub simuls: Option<Vec<String>>,
    pub correspondence_moves: Option<CorrespondenceMoves>,
    pub correspondence_ends: Option<CorrespondenceEnds>,
    pub follows: Option<FollowActivity>,
    pub studies: Option<serde_json::Value>, // OpenAPI shows empty object
    pub teams: Option<Vec<TeamActivity>>,
    pub posts: Option<Vec<PostActivity>>,
    pub patron: Option<PatronActivity>,
    pub stream: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityInterval {
    pub start: u64,
    pub end: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserActivityScore {
    pub win: u32,
    pub loss: u32,
    pub draw: u32,
    pub rp: RatingProgress,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RatingProgress {
    pub before: u32,
    pub after: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PuzzleActivity {
    pub score: UserActivityScore,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PuzzleModePerf {
    pub runs: u32,
    pub score: u32,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentActivity {
    pub nb: u32,
    pub best: Option<Vec<TournamentResult>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentResult {
    pub tournament: TournamentInfo,
    pub nb_games: u32,
    pub score: u32,
    pub rank: u32,
    pub rank_percent: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TournamentInfo {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PracticeActivity {
    pub url: String,
    pub name: String,
    pub nb_positions: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorrespondenceMoves {
    pub nb: u32,
    pub games: Vec<UserActivityCorrespondenceGame>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorrespondenceEnds {
    pub score: UserActivityScore,
    pub games: Vec<UserActivityCorrespondenceGame>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserActivityCorrespondenceGame {
    pub id: String,
    pub color: Color,
    pub url: String,
    pub variant: VariantKey,
    pub speed: String, // Always "correspondence"
    pub perf: String,  // Always "correspondence"
    pub rated: bool,
    pub opponent: CorrespondenceOpponent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorrespondenceOpponent {
    pub user: String,
    pub rating: u32,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FollowActivity {
    #[serde(rename = "in")]
    pub incoming: Option<UserActivityFollowList>,
    pub out: Option<UserActivityFollowList>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserActivityFollowList {
    pub ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb: Option<u32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamActivity {
    pub url: String,
    pub name: String,
    pub flair: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostActivity {
    pub topic_url: String,
    pub topic_name: String,
    pub posts: Vec<PostInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostInfo {
    pub url: String,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatronActivity {
    pub months: u32,
}