use crate::model::Request;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub nb: Option<u32>,
    pub status: Option<u8>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str, query: GetQuery) -> Self {
        Self::get(
            format!("/api/user/{username}/tournament/created"),
            query,
            None,
        )
    }
}
