use crate::model::Request;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub declined: Option<bool>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(team_id: &str, query: GetQuery) -> Self {
        Self::get(format!("/api/team/{team_id}/requests"), query, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(team_id: S) -> Self {
        Self::new(team_id.as_ref(), GetQuery::default())
    }
}
