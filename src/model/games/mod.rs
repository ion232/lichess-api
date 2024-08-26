pub mod export;
pub mod import;
pub mod ongoing;
pub mod stream;

use crate::model::{Clock, LightUser, PlayerColor, Speed, VariantKey};
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStream {
    pub id: String,
    pub rated: bool,
    pub variant: VariantKey,
    pub speed: Speed,
    pub perf: String,
    pub created_at: u64,
    pub status: u32,
    pub status_name: GameStatus,
    pub players: Players,
    pub clock: Option<Clock>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameJson {
    pub id: String,
    pub rated: bool,
    pub variant: VariantKey,
    pub speed: Speed,
    pub perf: String,
    pub created_at: u64,
    pub last_move_at: Option<u64>,
    pub status: GameStatus,
    pub players: Option<Players>,
    pub initial_fen: Option<String>,
    pub winner: Option<PlayerColor>,
    pub opening: Option<Opening>,
    pub moves: Option<String>,
    pub pgn: Option<String>,
    pub days_per_turn: Option<u64>,
    pub analysis: Option<GameMoveAnalysis>,
    pub tournament: Option<String>,
    pub swiss: Option<String>,
    pub clock: Option<Clock>,
    pub clocks: Option<Vec<u64>>,
    pub division: Option<Division>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GameStatus {
    Created,
    Started,
    Aborted,
    Mate,
    Resign,
    Stalemate,
    Timeout,
    Draw,
    Outoftime,
    Cheat,
    NoStart,
    UnknownFinish,
    VariantEnd,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct GameStatusJson {
    pub id: u32,
    pub name: GameStatus,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Players {
    pub white: Option<GameUser>,
    pub black: Option<GameUser>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameUser {
    pub user: Option<LightUser>,
    pub rating: Option<u32>,
    pub rating_diff: Option<i32>,
    pub name: Option<String>,
    pub provisional: Option<bool>,
    pub ai_level: Option<u32>,
    pub analysis: Option<Analysis>,
    pub team: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub inaccuracy: u32,
    pub mistake: u32,
    pub blunder: u32,
    pub acpl: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opening {
    pub eco: String,
    pub name: String,
    pub ply: u32,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Division {
    pub middle: Option<u32>,
    pub end: Option<u32>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameMoveAnalysis {
    pub eval: Option<u32>,
    pub mate: Option<u32>,
    pub best: Option<String>,
    pub variation: Option<String>,
    // ion232: Incorrect spelling here is expected.
    pub judgment: Option<Judgement>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judgement {
    pub name: JudgementName,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JudgementName {
    // ion232: These are expected to be uppercase.
    Inaccuracy,
    Mistake,
    Blunder,
}
