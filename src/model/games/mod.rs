pub mod export;
pub mod import;
pub mod ongoing;
pub mod stream;

use crate::model::{LightUser, Speed, Variant, VariantKey};
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
    pub variant: Variant,
    pub speed: Speed,
    pub perf: String,
    pub created_at: u64,
    pub last_move_at: Option<u64>,
    pub status: GameStatusJson,
    pub players: Option<Players>,
    pub player: Option<String>,
    pub opening: Option<Opening>,
    pub moves: Option<String>,
    pub clock: Option<Clock>,
    pub winner: Option<String>,
    pub fen: Option<String>,
    pub turns: Option<u32>,
    pub source: Option<String>,
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
    pub provisional: Option<bool>,
    pub rating: Option<u32>,
    pub rating_diff: Option<i32>,
    pub name: Option<String>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    pub initial: u32,
    pub increment: u32,
    pub total_time: Option<u64>,
}
