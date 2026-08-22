use crate::model::Request;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub nb: Option<u32>,
    pub html: Option<bool>,
    pub live: Option<bool>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self::get("/api/broadcasts/official", query, None)
    }
}

impl From<GetQuery> for GetRequest {
    fn from(query: GetQuery) -> Self {
        Self::new(query)
    }
}
