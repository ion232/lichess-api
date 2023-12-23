use super::Base;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    #[serde(flatten)]
    pub base: Base,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str, query: GetQuery) -> Self {
        let path = format!("/api/user/{}/current-game", username);
        Self {
            path,
            query: Some(query),
            ..Default::default()
        }
    }
}
