pub mod by_ids;
pub mod by_user;
pub mod one;
pub mod ongoing;

use serde::Serialize;

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    pub moves: bool,
    pub pgn_in_json: bool,
    pub tags: bool,
    pub clocks: bool,
    pub evals: bool,
    pub accuracy: bool,
    pub opening: bool,
    pub literate: bool,
    pub players: Option<String>,
}

impl Default for Base {
    fn default() -> Self {
        Base {
            moves: true,
            pgn_in_json: false,
            tags: true,
            clocks: false,
            evals: false,
            accuracy: false,
            opening: false,
            literate: false,
            players: None,
        }
    }
}
