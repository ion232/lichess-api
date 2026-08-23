use crate::model::{Clock, Room, Speed, Variant};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GameColor {
    White,
    Black,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GameStatusName {
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
// Boxing would ripple through every match arm across the crate for a rarely-streamed enum; not worth it.
#[allow(clippy::large_enum_variant)]
pub enum Event {
    GameFull {
        #[serde(flatten)]
        game_full: GameFullEvent,
    },
    GameState {
        #[serde(flatten)]
        game_state: GameStateEvent,
    },
    ChatLine {
        #[serde(flatten)]
        chat_line: ChatLineEvent,
    },
    OpponentGone {
        #[serde(flatten)]
        opponent_gone: OpponentGoneEvent,
    },
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameFullEvent {
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
    pub state: Option<GameStateEvent>,
    pub tournament_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStateEvent {
    // Will always be gameState, but needed to avoid cycles.
    pub r#type: Option<String>,
    pub moves: String,
    pub wtime: u64,
    pub btime: u64,
    pub winc: u64,
    pub binc: u64,
    pub status: GameStatusName,
    pub winner: Option<GameColor>,
    pub wdraw: Option<bool>,
    pub bdraw: Option<bool>,
    pub wtakeback: Option<bool>,
    pub btakeback: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatLineEvent {
    pub room: Room,
    pub username: String,
    pub text: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpponentGoneEvent {
    pub gone: bool,
    pub claim_win_in_seconds: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameEventPlayer {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisional: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Perf {
    pub name: String,
}
