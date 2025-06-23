use crate::model::challenges::{ChallengeDeclinedJson, ChallengeJson};
use crate::model::{Color, GameCompat, Speed, Variant};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self::get("/api/stream/event", None, None)
    }
}

impl Default for GetRequest {
    fn default() -> Self {
        Self::new()
    }
}

// Response structs.

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    #[serde(flatten)]
    pub event: EventData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compat: Option<GameCompat>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum EventData {
    Challenge { challenge: ChallengeJson },
    ChallengeCanceled { challenge: ChallengeJson },
    ChallengeDeclined { challenge: ChallengeDeclinedJson },
    GameStart { game: GameEventInfo },
    GameFinish { game: GameEventInfo },
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameEventInfo {
    pub full_id: String,
    pub game_id: String,
    pub fen: String,
    pub color: Color,
    pub last_move: String,
    pub source: Source,
    pub status: GameStatus,
    pub variant: Variant,
    pub speed: Speed,
    pub perf: String,
    pub rated: bool,
    pub has_moved: bool,
    pub opponent: GameEventOpponent,
    pub is_my_turn: bool,
    pub seconds_left: Option<u64>,
    pub tournament_id: Option<String>,
    pub compat: Option<GameCompat>,
    pub winner: Option<String>,
    pub rating_diff: Option<i32>,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameStatus {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Source {
    Lobby,
    Friend,
    Ai,
    Api,
    Tournament,
    Position,
    Import,
    Importlive,
    Simul,
    Relay,
    Pool,
    Swiss,
    Arena,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all_fields = "camelCase")]
pub enum GameEventOpponent {
    Player {
        id: String,
        username: String,
        rating: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        rating_diff: Option<i32>,
    },
    AI {
        id: Option<String>, // Always null for AI
        username: String,
        ai: u32,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChallengeCanceledJson {
    pub id: String,
}
