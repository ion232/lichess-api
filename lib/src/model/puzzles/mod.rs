pub mod activity;
pub mod batch;
pub mod daily;
pub mod dashboard;
pub mod id;
pub mod next;
pub mod race;
pub mod racer;
pub mod replay;
pub mod storm_dashboard;

use super::Title;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleAndGame {
    pub game: Game,
    pub puzzle: Puzzle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub clock: String,
    pub id: String,
    pub perf: Perf,
    pub pgn: String,
    pub players: Vec<Player>,
    pub rated: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Puzzle {
    pub id: String,
    #[serde(rename = "initialPly")]
    pub initial_ply: i32,
    pub plays: i32,
    pub rating: i32,
    pub fen: Option<String>,
    #[serde(rename = "lastMove")]
    pub last_move: Option<String>,
    pub solution: Vec<String>,
    pub themes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Perf {
    pub key: String,
    pub name: String,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub color: String,
    pub id: String,
    pub name: String,
    pub rating: i32,
    pub flair: Option<String>,
    pub patron: Option<bool>,
    #[serde(rename = "patronColor")]
    pub patron_color: Option<u32>,
    pub title: Option<Title>,
}
