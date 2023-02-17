use crate::model::challenges::ChallengeJson;
use crate::model::{Color, Compat, Speed, Variant};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self {
            method: http::Method::GET,
            path: "/api/stream/event".to_string(),
            query: Default::default(),
            body: Default::default(),
        }
    }
}

// Response structs.

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Event {
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
    pub game_id: String,
    pub full_id: String,
    pub color: Color,
    pub fen: String,
    pub has_moved: bool,
    pub is_my_turn: bool,
    pub last_move: String,
    pub opponent: Opponent,
    pub perf: String,
    pub rated: bool,
    pub seconds_left: u64,
    pub source: Source,
    pub speed: Speed,
    pub variant: Variant,
    pub compat: Option<Compat>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opponent {
    pub id: String,
    pub rating: u32,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChallengeCanceledJson {
    pub id: String,
}
