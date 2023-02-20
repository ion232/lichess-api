use crate::model::{Room, Speed, Variant};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(game_id: &str) -> Self {
        let path = format!("/api/board/game/stream/{}", game_id);
        Self {
            method: http::Method::GET,
            path,
            query: Default::default(),
            body: Default::default(),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Event {
    #[serde(rename_all = "camelCase")]
    GameFull {
        id: String,
        variant: Variant,
        rated: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        clock: Option<Clock>,
        speed: Speed,
        perf: Perf,
        created_at: u64,
        white: GameEventPlayer,
        black: GameEventPlayer,
        #[serde(skip_serializing_if = "Option::is_none")]
        initial_fen: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<GameState>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tournament_id: Option<String>,
    },
    GameState {
        #[serde(flatten)]
        state: GameState,
    },
    ChatLine {
        room: Room,
        username: String,
        text: String,
    },
    #[serde(rename_all = "camelCase")]
    OpponentGone {
        gone: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        claim_win_in_seconds: Option<u32>,
    },
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "gameState")]
pub struct GameState {
    pub moves: String,
    pub wtime: u64,
    pub btime: u64,
    pub winc: u64,
    pub binc: u64,
    pub status: String,
    pub winner: Option<String>,
    pub wdraw: Option<bool>,
    pub bdraw: Option<bool>,
    pub wtakeback: Option<bool>,
    pub btakeback: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameEventPlayer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_level: Option<u32>,
    pub name: String,
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisional: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clock {
    pub initial: u32,
    pub increment: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Perf {
    pub name: String,
}
