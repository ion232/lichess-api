use serde::{Deserialize, Serialize};

use crate::model::{Domain, Request, VariantKey};

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

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self::get("/lichess", query, Domain::Explorer)
    }
}

impl From<GetQuery> for GetRequest {
    fn from(query: GetQuery) -> Self {
        Self::new(query)
    }
}
