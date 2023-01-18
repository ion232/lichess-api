// /api/puzzle/activity
// /api/puzzle/daily
// /api/puzzle/dashboard/{days}
// /api/puzzle/{id}
// /api/racer
// /api/storm/dashboard/{username}

// struct PGN; // Use PGN parser for this.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Perf {
    pub key: String,
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub color: String,
    pub name: String,
    #[serde(rename = "userId")] 
    pub user_id: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Game {
    pub clock: String,
    pub id: String,
    pub perf: Perf,
    pub pgn: String,
    pub players: Vec<Player>,
    pub rated: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Puzzle {
    pub id: String,
    #[serde(rename = "initialPly")] 
    pub initial_ply: i32,
    pub plays: i32,
    pub rating: i32,
    pub solution: Vec<String>,
    pub themes: Vec<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PuzzleJson {
    pub game: Game,
    pub puzzle: Puzzle
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PuzzleRoundJson {
    pub id: String,
    pub date: u64,
    pub win: bool,
    #[serde(rename = "puzzleRating")] 
    pub puzzle_rating: u32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DailyPuzzleQuery;

pub type DailyPuzzle = PuzzleJson;
