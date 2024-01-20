use serde::{Deserialize, Serialize};

use crate::model::VariantKey;

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde_with::skip_serializing_none]
pub struct GetQuery {
    pub variant: VariantKey,
    pub fen: String,
    pub play: String,
    pub speeds: Option<String>,
    pub ratings: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub moves: Option<u32>,
    pub top_games: Option<u32>,
    pub recent_games: Option<u32>,
    pub history: Option<bool>,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self {
            domain: crate::model::Domain::Explorer,
            path: "/lichess".to_string(),
            query: Some(query),
            ..Default::default()
        }
    }
}
