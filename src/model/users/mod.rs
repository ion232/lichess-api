pub mod public;
pub mod status;

use crate::model::Title;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserExtended {
    pub url: String,
    pub playing: String,
    pub completion_rate: u32,
    pub count: Count,
    pub streaming: bool,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub perfs: Perfs,
    pub created_at: i64,
    pub disabled: bool,
    pub tos_violation: bool,
    pub profile: Profile,
    pub seen_at: i64,
    pub patron: bool,
    pub verified: bool,
    pub play_time: PlayTime,
    pub title: Title,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Perfs {
    pub chess960: Perf,
    pub atomic: Perf,
    pub racing_kings: Perf,
    pub ultra_bullet: Perf,
    pub blitz: Perf,
    pub king_of_the_hill: Perf,
    pub bullet: Perf,
    pub correspondence: Perf,
    pub horde: Perf,
    pub puzzle: Perf,
    pub classical: Perf,
    pub rapid: Perf,
    pub storm: Storm,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Perf {
    pub games: u32,
    pub rating: u32,
    pub rd: u32,
    pub prog: i32,
    pub prov: bool,
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
    pub country: String,
    pub location: String,
    pub bio: String,
    pub first_name: String,
    pub last_name: String,
    pub fide_rating: u32,
    pub uscf_rating: u32,
    pub ecf_rating: u32,
    pub links: String,
}
