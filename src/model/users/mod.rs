pub mod activity;
pub mod autocomplete;
pub mod by_id;
pub mod crosstable;
pub mod leaderboard;
pub mod live_streamers;
pub mod note;
pub mod performance;
pub mod public;
pub mod rating_history;
pub mod status;
pub mod top_10;

use std::collections::HashMap;

use crate::model::{LightUser, Title};
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Stream {
    pub service: String,
    pub status: String,
    pub lang: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Streamer {
    pub name: String,
    pub headline: String,
    pub description: Option<String>,
    #[serde(rename = "youTube")]
    pub youtube: Option<String>,
    pub twitch: Option<String>,
    pub image: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamingUser {
    #[serde(flatten)]
    pub user: LightUser,
    pub stream: Stream,
    pub streamer: Streamer,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Matchup {
    pub users: HashMap<String, f64>,

    #[serde(rename = "nbGames")]
    pub game_count: u32,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Crosstable {
    #[serde(flatten)]
    pub all_time: Matchup,
    pub matchup: Option<Matchup>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    from: LightUser,
    to: LightUser,
    text: String,
    date: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LeaderboardPerf {
    pub rating: u32,
    pub progress: i32,
}

// Lichess returns { perfs: { "{PerfType}": { rating: u32, progress: u32} } },
// this therfore more accurate than a struct with `variant_name: Perf` for each variant.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LeaderboardPerfs {
    Bullet(LeaderboardPerf),
    Blitz(LeaderboardPerf),
    Rapid(LeaderboardPerf),
    Classical(LeaderboardPerf),
    UltraBullet(LeaderboardPerf),
    Chess960(LeaderboardPerf),
    Crazyhouse(LeaderboardPerf),
    Antichess(LeaderboardPerf),
    Atomic(LeaderboardPerf),
    Horde(LeaderboardPerf),
    KingOfTheHill(LeaderboardPerf),
    RacingKings(LeaderboardPerf),
    ThreeCheck(LeaderboardPerf),
}

impl LeaderboardPerfs {
    pub fn into_perf(self) -> LeaderboardPerf {
        match self {
            Self::Bullet(perf)
            | Self::Blitz(perf)
            | Self::Rapid(perf)
            | Self::Classical(perf)
            | Self::UltraBullet(perf)
            | Self::Chess960(perf)
            | Self::Crazyhouse(perf)
            | Self::Antichess(perf)
            | Self::Atomic(perf)
            | Self::Horde(perf)
            | Self::KingOfTheHill(perf)
            | Self::RacingKings(perf)
            | Self::ThreeCheck(perf) => perf,
        }
    }
}

impl From<LeaderboardPerfs> for LeaderboardPerf {
    fn from(perfs: LeaderboardPerfs) -> LeaderboardPerf {
        perfs.into_perf()
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub username: String,
    pub perfs: LeaderboardPerfs,
    pub title: Option<Title>,
    pub patron: Option<bool>,
    pub online: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboards {
    pub bullet: Vec<Player>,
    pub blitz: Vec<Player>,
    pub rapid: Vec<Player>,
    pub classical: Vec<Player>,
    pub ultra_bullet: Vec<Player>,
    pub chess960: Vec<Player>,
    pub crazyhouse: Vec<Player>,
    pub antichess: Vec<Player>,
    pub atomic: Vec<Player>,
    pub horde: Vec<Player>,
    pub king_of_the_hill: Vec<Player>,
    pub racing_kings: Vec<Player>,
    pub three_check: Vec<Player>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Leaderboard {
    pub users: Vec<Player>,
}
