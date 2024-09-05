pub mod channels;
pub mod stream;

use crate::model::{Color, LightUser};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
    pub user: LightUser,
    pub rating: u32,
    pub game_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channels {
    bot: Champion,
    blitz: Champion,
    racing_kings: Champion,
    ultra_bullet: Champion,
    bullet: Champion,
    classical: Champion,
    three_check: Champion,
    antichess: Champion,
    computer: Champion,
    horde: Champion,
    rapid: Champion,
    atomic: Champion,
    crazyhouse: Champion,
    chess960: Champion,
    king_of_the_hill: Champion,
    best: Champion,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChannelName {
    Bot,
    Blitz,
    RacingKings,
    UltraBullet,
    Bullet,
    Classical,
    ThreeCheck,
    Antichess,
    Computer,
    Horde,
    Rapid,
    Atomic,
    Crazyhouse,
    Chess960,
    KingOfTheHill,
    Best,
}

impl ChannelName {
    pub const fn as_str(self) -> &'static str {
        match self {
            ChannelName::Bot => "bot",
            ChannelName::Blitz => "blitz",
            ChannelName::RacingKings => "racingKings",
            ChannelName::UltraBullet => "ultraBullet",
            ChannelName::Bullet => "bullet",
            ChannelName::Classical => "classical",
            ChannelName::ThreeCheck => "threeCheck",
            ChannelName::Antichess => "antichess",
            ChannelName::Computer => "computer",
            ChannelName::Horde => "horde",
            ChannelName::Rapid => "rapid",
            ChannelName::Atomic => "atomic",
            ChannelName::Crazyhouse => "crazyhouse",
            ChannelName::Chess960 => "chess960",
            ChannelName::KingOfTheHill => "kingOfTheHill",
            ChannelName::Best => "best",
        }
    }
}

impl std::fmt::Display for ChannelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
