use crate::model::{Clock, Room, Speed, Variant};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(game_id: &str) -> Self {
        Self::get(format!("/api/board/game/stream/{game_id}"), None, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(s: S) -> Self {
        Self::new(s.as_ref())
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Event {
    GameFull {
        #[serde(flatten)]
        game_full: GameFull,
    },
    GameState {
        #[serde(flatten)]
        game_state: GameState,
    },
    ChatLine {
        #[serde(flatten)]
        chat_line: ChatLine,
    },
    OpponentGone {
        #[serde(flatten)]
        opponent_gone: OpponentGone,
    },
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameFull {
    pub id: String,
    pub variant: Variant,
    pub rated: bool,
    pub clock: Option<Clock>,
    pub speed: Speed,
    pub perf: Perf,
    pub created_at: u64,
    pub white: GameEventPlayer,
    pub black: GameEventPlayer,
    pub initial_fen: Option<String>,
    pub state: Option<GameState>,
    pub tournament_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    // Will always be gameState, but needed to avoid cycles.
    pub r#type: Option<String>,
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

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatLine {
    pub room: Room,
    pub username: String,
    pub text: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpponentGone {
    pub gone: bool,
    pub claim_win_in_seconds: Option<u32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum GameEventPlayer {
    AI {
        #[serde(flatten)]
        ai: GameEventAI,
    },
    Human {
        #[serde(flatten)]
        human: GameEventHuman,
    },
    Anonymous {},
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameEventAI {
    pub ai_level: u32,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameEventHuman {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub rating: Option<u32>,
    pub provisional: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Perf {
    pub name: String,
}
