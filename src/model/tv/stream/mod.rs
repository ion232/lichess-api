pub mod channel;
pub mod current;

use crate::model::{Color, LightUser};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    pub user: LightUser,
    pub rating: u32,
    pub seconds: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "t", content = "d", rename_all = "camelCase")]
pub enum Event {
    Featured {
        id: String,
        orientation: Color,
        players: [StreamUser; 2],
        fen: String,
    },

    Fen {
        fen: String,
        #[serde(rename = "lm")]
        #[serde(skip_serializing_if = "Option::is_none")]
        last_move: Option<String>,
        #[serde(rename = "wc")]
        #[serde(skip_serializing_if = "Option::is_none")]
        white_centipawns: Option<u32>,
        #[serde(rename = "bc")]
        #[serde(skip_serializing_if = "Option::is_none")]
        black_centipawns: Option<u32>,
    },
}
