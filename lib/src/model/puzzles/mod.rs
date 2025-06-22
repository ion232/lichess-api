pub mod activity;
pub mod daily;
pub mod dashboard;
pub mod id;
pub mod next;
pub mod race;
pub mod replay;
pub mod storm_dashboard;

use serde::{Deserialize, Serialize};
use super::Title;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Puzzle {
    pub id: String,
    #[serde(rename = "initialPly")]
    pub initial_ply: i32,
    pub plays: i32,
    pub rating: i32,
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
    pub title: Option<Title>,
}
