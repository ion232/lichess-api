use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Default, Clone, Debug, Serialize)]
#[skip_serializing_none]
pub struct GetQuery {
    #[serde(rename = "q")]
    pub query: String,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self::get("/api/fide/player", query, None)
    }
}

impl From<&str> for GetRequest {
    fn from(query: &str) -> Self {
        GetQuery { query: query.to_string() }.into()
    }
}

impl From<GetQuery> for GetRequest {
    fn from(query: GetQuery) -> Self {
        Self::new(query)
    }
}

