use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde_with::skip_serializing_none]
pub struct GetQuery {
    pub fen: String,
    pub play: String,
    pub since: Option<u32>,
    pub until: Option<u32>,
    pub moves: Option<u32>,
    pub top_games: Option<u32>,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self {
            domain: crate::model::Domain::Explorer,
            path: "/masters".to_string(),
            query: Some(query),
            ..Default::default()
        }
    }
}
