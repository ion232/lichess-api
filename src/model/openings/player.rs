use serde::{Deserialize, Serialize};

use crate::model::{Color, VariantKey};

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
        Self {
            domain: crate::model::Domain::Explorer,
            path: "/player".to_string(),
            query: Some(query),
            ..Default::default()
        }
    }
}
