pub mod all;
pub mod arena;
pub mod join;
pub mod kick;
pub mod of_username;
pub mod pm_all;
pub mod quit;
pub mod request_accept;
pub mod request_decline;
pub mod requests;
pub mod search;
pub mod show;
pub mod swiss;
pub mod updates;
pub mod updates_of_team;
pub mod users;

use crate::model::LightUser;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub flair: Option<String>,
    pub leader: Option<LightUser>,
    pub leaders: Option<Vec<LightUser>>,
    #[serde(rename = "nbMembers")]
    pub nb_members: Option<u32>,
    pub open: Option<bool>,
    pub joined: Option<bool>,
    pub requested: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LightTeam {
    pub id: String,
    pub name: String,
    pub flair: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamPaginator<T> {
    pub current_page: u32,
    pub max_per_page: u32,
    pub current_page_results: Vec<T>,
    pub previous_page: Option<u32>,
    pub next_page: Option<u32>,
    pub nb_results: u32,
    pub nb_pages: u32,
}

pub type TeamPaginatorJson = TeamPaginator<Team>;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamRequest {
    pub team_id: String,
    pub user_id: String,
    pub date: u64,
    pub message: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamRequestWithUser {
    pub request: TeamRequest,
    pub user: crate::model::users::User,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamUpdate {
    pub msg: TeamUpdateMessage,
    pub seen: bool,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamUpdateMessage {
    pub id: String,
    pub date: u64,
    pub sender: LightUser,
    pub team: LightTeam,
    pub text: String,
}

pub type TeamUpdatesPager = TeamPaginator<TeamUpdate>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamUpdatesByTeamEntry {
    pub team: LightTeam,
    pub last: u64,
    pub unread: u32,
}

pub type TeamUpdatesByTeam = Vec<TeamUpdatesByTeamEntry>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamUpdates {
    pub updates: TeamUpdatesPager,
    #[serde(rename = "byTeam")]
    pub by_team: TeamUpdatesByTeam,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamUpdatesOfTeam {
    pub team: LightTeam,
    pub subscribed: bool,
    pub updates: TeamUpdatesPager,
    #[serde(rename = "byTeam")]
    pub by_team: TeamUpdatesByTeam,
}
