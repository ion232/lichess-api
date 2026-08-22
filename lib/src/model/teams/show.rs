use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(team_id: &str) -> Self {
        Self::get(format!("/api/team/{team_id}"), None, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(team_id: S) -> Self {
        Self::new(team_id.as_ref())
    }
}
