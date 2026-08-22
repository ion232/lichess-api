use super::Base;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    #[serde(flatten)]
    pub base: Base,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Sort {
    DateAsc,
    DateDesc,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self::get("/api/games/export/bookmarks", query, None)
    }
}
