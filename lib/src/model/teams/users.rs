use crate::model::{Request, Title};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub full: Option<bool>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(team_id: &str, query: GetQuery) -> Self {
        Self::get(format!("/api/team/{team_id}/users"), query, None)
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamMember {
    pub id: String,
    pub name: String,
    #[serde(rename = "joinedTeamAt")]
    pub joined_team_at: u64,
    pub title: Option<Title>,
    #[serde(rename = "patronColor")]
    pub patron_color: Option<u8>,
}
