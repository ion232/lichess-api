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
        let path = format!("/game/export/{}", game_id);
        Self {
            method: http::Method::GET,
            path,
            query: Some(query),
            body: Default::default(),
        }
    }
}
