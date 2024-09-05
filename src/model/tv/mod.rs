pub mod channels;

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
