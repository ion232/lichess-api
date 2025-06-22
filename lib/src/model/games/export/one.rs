use super::Base;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    #[serde(flatten)]
    pub base: Base,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(game_id: &str, query: GetQuery) -> Self {
        Self::get(format!("/game/export/{game_id}"), query, None)
    }
}
