use crate::model::Request;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub player: Option<String>,
    pub moves: Option<bool>,
    #[serde(rename = "pgnInJson")]
    pub pgn_in_json: Option<bool>,
    pub tags: Option<bool>,
    pub clocks: Option<bool>,
    pub evals: Option<bool>,
    pub accuracy: Option<bool>,
    pub opening: Option<bool>,
    pub division: Option<bool>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(id: &str, query: GetQuery) -> Self {
        Self::get(format!("/api/swiss/{id}/games"), query, None)
    }
}
