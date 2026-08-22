use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(broadcast_tournament_id: &str) -> Self {
        Self::get(
            format!("/broadcast/{broadcast_tournament_id}/teams/standings"),
            None,
            None,
        )
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(broadcast_tournament_id: S) -> Self {
        Self::new(broadcast_tournament_id.as_ref())
    }
}
