pub mod leaderboard;
pub mod top_10;

use crate::model::Title;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Perf {
    pub rating: u32,
    pub progress: i32,
}

// Lichess returns { perfs: { "{PerfType}": { rating: u32, progress: u32} } },
// this therfore more accurate than a struct with `variant_name: Perf` for each variant.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Perfs {
    Bullet(Perf),
    Blitz(Perf),
    Rapid(Perf),
    Classical(Perf),
    UltraBullet(Perf),
    Chess960(Perf),
    Crazyhouse(Perf),
    Antichess(Perf),
    Atomic(Perf),
    Horde(Perf),
    KingOfTheHill(Perf),
    RacingKings(Perf),
    ThreeCheck(Perf),
}

impl Perfs {
    pub fn into_perf(self) -> Perf {
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

impl From<Perfs> for Perf {
    fn from(perfs: Perfs) -> Perf {
        perfs.into_perf()
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub username: String,
    pub perfs: Perfs,
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
