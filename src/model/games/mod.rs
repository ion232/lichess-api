pub mod export;
pub mod import;
pub mod ongoing;
pub mod stream;

use crate::model::{LightUser, Speed, VariantKey};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameJson {
    id: String,
    rated: bool,
    variant: VariantKey,
    speed: Speed,
    perf: String,
    created_at: u64,
    last_move_at: u64,
    status: GameStatus,
    players: Players,
    opening: Opening,
    moves: String,
    clock: Clock,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Players {
    pub white: GameUser,
    pub black: GameUser,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameUser {
    pub user: LightUser,
    pub rating: u32,
    pub rating_diff: i32,
    pub name: String,
    pub provisional: bool,
    pub ai_level: u32,
    pub analysis: Analysis,
    pub team: String,
}

#[derive(Serialize, Deserialize)]
pub struct Analysis {
    pub inaccuracy: u32,
    pub mistake: u32,
    pub blunder: u32,
    pub acpl: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Opening {
    pub eco: String,
    pub name: String,
    pub ply: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    pub initial: u32,
    pub increment: u32,
    pub total_time: u64,
}
