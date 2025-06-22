use serde::{Deserialize, Serialize};

use crate::model::{Domain, Request};

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

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self::get("/masters", query, Domain::Explorer)
    }
}

impl From<GetQuery> for GetRequest {
    fn from(query: GetQuery) -> Self {
        Self::new(query)
    }
}
