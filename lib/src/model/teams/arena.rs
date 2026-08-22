use crate::model::{ArenaStatusName, Request};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub max: Option<u32>,
    pub status: Option<ArenaStatusName>,
    pub created_by: Option<String>,
    pub name: Option<String>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(team_id: &str, query: GetQuery) -> Self {
        Self::get(format!("/api/team/{team_id}/arena"), query, None)
    }
}
