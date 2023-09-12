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
    id: String,
    rated: bool,
    variant: VariantKey,
    speed: Speed,
    perf: String,
    created_at: u64,
    status: u32,
    status_name: GameStatus,
    players: Players,
    clock: Option<Clock>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameJson {
    id: String,
    rated: bool,
    variant: Variant,
    speed: Speed,
    perf: String,
    created_at: u64,
    last_move_at: Option<u64>,
    status: GameStatusJson,
    players: Option<Players>,
    player: Option<String>,
    opening: Option<Opening>,
    moves: Option<String>,
    clock: Option<Clock>,
    winner: Option<String>,
    fen: Option<String>,
    turns: Option<u32>,
    source: Option<String>,
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
