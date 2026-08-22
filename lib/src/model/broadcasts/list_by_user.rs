use crate::model::Request;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub page: Option<u32>,
    pub html: Option<bool>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str, query: GetQuery) -> Self {
        Self::get(format!("/api/broadcasts/by/{username}"), query, None)
    }
}
