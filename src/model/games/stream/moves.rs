use crate::model::Variant;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(id: &str) -> Self {
        let path = format!("/api/stream/game/{}", id);
        Self {
            method: http::Method::GET,
            path,
            query: Default::default(),
            body: Default::default(),
        }
    }
}

pub type Move = MoveStream;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MoveStream {
    #[serde(rename_all = "camelCase")]
    Start {
        id: String,
        variant: Variant,
        speed: String,
        perf: String,
        rated: bool,
        initial_fen: String,
        fen: String,
        player: String,
        turns: u32,
        started_at_turn: u64,
        source: String,
        status: Status,
        created_at: u64,
        last_move: String,
    },
    Move {
        fen: String,
        #[serde(rename = "lm")]
        last_move: String,
        #[serde(rename = "wc")]
        white_centipawns: u32,
        #[serde(rename = "bc")]
        black_centipawns: u32,
    },
    End {
        description: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub id: u64,
    pub name: String,
}
