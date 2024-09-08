use serde::{Deserialize, Serialize};

use crate::model::{Color, Domain, VariantKey};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde_with::skip_serializing_none]
pub struct GetQuery {
    pub player: String,
    pub fen: String,
    pub color: Color,
    pub play: String,
    pub variant: VariantKey,
    pub speeds: Option<String>,
    pub modes: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub moves: Option<u32>,
    pub recent_games: Option<u32>,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self::get("/player", Some(query), Some(Domain::Explorer))
    }
}

impl From<GetQuery> for GetRequest {
    fn from(query: GetQuery) -> Self {
        Self::new(query)
    }
}
