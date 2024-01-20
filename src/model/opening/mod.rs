pub mod lichess;
pub mod masters;
pub mod otb;

use serde::{Deserialize, Serialize};

use crate::model::Color;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct OpeningExplorerJson {
    pub opening: Option<Opening>,
    pub white: u32,
    pub draws: u32,
    pub black: u32,
    pub moves: Vec<Move>,
    pub top_games: Vec<Game>,
    pub recent_games: Option<Vec<Game>>,
    pub history: Option<Vec<History>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opening {
    pub eco: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Move {
    pub uci: String,
    pub san: String,
    pub average_rating: Option<u32>,
    pub white: u32,
    pub draws: u32,
    pub black: u32,
    pub game: Option<Game>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct Game {
    pub id: String,
    pub winner: Option<Color>,
    pub white: Player,
    pub black: Player,
    pub year: u32,
    pub month: String,
    pub uci: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub rating: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct History {
    pub month: String,
    pub black: u32,
    pub draws: u32,
    pub white: u32,
}
