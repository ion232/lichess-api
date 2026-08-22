use crate::model::Request;
use serde::Serialize;

#[serde_with::skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub text: Option<String>,
    pub page: Option<u32>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self::get("/api/team/search", query, None)
    }
}

impl From<GetQuery> for GetRequest {
    fn from(query: GetQuery) -> Self {
        Self::new(query)
    }
}
