use crate::model::games::Players;
use crate::model::Variant;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(id: &str) -> Self {
        let path = format!("/api/stream/game/{}", id);
        Self {
            path,
            ..Default::default()
        }
    }
}

pub type Move = MoveStream;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MoveStream {
    #[serde(rename_all = "camelCase")]
    Start {
        id: String,
        variant: Variant,
        speed: String,
        perf: String,
        rated: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        initial_fen: Option<String>,
        fen: String,
        player: String,
        turns: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        started_at_turn: Option<u64>,
        source: String,
        status: Status,
        created_at: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_move: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        players: Option<Players>,
    },
    Move {
        fen: String,
        #[serde(rename = "lm")]
        #[serde(skip_serializing_if = "Option::is_none")]
        last_move: Option<String>,
        #[serde(rename = "wc")]
        #[serde(skip_serializing_if = "Option::is_none")]
        white_centipawns: Option<u32>,
        #[serde(rename = "bc")]
        #[serde(skip_serializing_if = "Option::is_none")]
        black_centipawns: Option<u32>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub id: u64,
    pub name: String,
}
