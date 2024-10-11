use crate::model::challenges::ChallengeJson;
use crate::model::{Color, Compat, Speed, Variant};
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
    pub event: InnerEvent,
    pub compat: Option<Compat>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum EventData {
    Challenge { challenge: ChallengeJson },
    ChallengeCanceled { challenge: ChallengeJson },
    ChallengeDeclined { challenge: ChallengeJson },
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
    pub variant: Variant,
    pub speed: Speed,
    pub perf: String,
    pub rated: bool,
    pub has_moved: bool,
    pub opponent: Opponent,
    pub is_my_turn: bool,
    pub seconds_left: Option<u64>,
    pub compat: Option<Compat>,
    pub winner: Option<String>,
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
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opponent {
    pub id: Option<String>,
    pub username: String,
    pub rating: Option<u32>,
    pub ai: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChallengeCanceledJson {
    pub id: String,
}
