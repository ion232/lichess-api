use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    matchup: Option<bool>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(user1: &str, user2: &str, matchup: Option<bool>) -> Self {
        let path = format!("/api/crosstable/{user1}/{user2}");
        Self::get(path, GetQuery { matchup }, None)
    }
}
