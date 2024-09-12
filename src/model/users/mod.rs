pub mod activity;
pub mod by_id;
pub mod performance;
pub mod public;
pub mod rating_history;
pub mod status;

use crate::model::Title;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserExtended {
    #[serde(flatten)]
    pub user: User,
    pub url: String,
    pub playing: Option<String>,
    pub completion_rate: Option<u32>,
    pub count: Count,
    pub streaming: Option<bool>,
    pub followable: bool,
    pub following: bool,
    pub blocking: bool,
    pub follows_you: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub all: u32,
    pub rated: u32,
    pub ai: u32,
    pub draw: u32,
    pub draw_h: u32,
    pub loss: u32,
    pub loss_h: u32,
    pub win: u32,
    pub win_h: u32,
    pub bookmark: u32,
    pub playing: u32,
    pub import: u32,
    pub me: u32,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub perfs: Perfs,
    pub created_at: i64,
    pub disabled: Option<bool>,
    pub tos_violation: Option<bool>,
    pub profile: Option<Profile>,
    pub seen_at: i64,
    pub patron: Option<bool>,
    pub verified: Option<bool>,
    pub play_time: PlayTime,
    pub title: Option<Title>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Perfs {
    pub chess960: Option<Perf>,
    pub atomic: Option<Perf>,
    pub racing_kings: Option<Perf>,
    pub ultra_bullet: Option<Perf>,
    pub blitz: Option<Perf>,
    pub king_of_the_hill: Option<Perf>,
    pub bullet: Option<Perf>,
    pub correspondence: Option<Perf>,
    pub horde: Option<Perf>,
    pub puzzle: Option<Perf>,
    pub classical: Option<Perf>,
    pub rapid: Option<Perf>,
    pub streak: Option<Storm>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Perf {
    pub games: u32,
    pub rating: u32,
    pub rd: u32,
    pub prog: i32,
    pub prov: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Storm {
    pub runs: u32,
    pub score: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayTime {
    pub total: u32,
    pub tv: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub country: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub fide_rating: Option<u32>,
    pub uscf_rating: Option<u32>,
    pub ecf_rating: Option<u32>,
    pub links: Option<String>,
}
