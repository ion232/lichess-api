use crate::model::{PerfType, Title};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Default, Clone, Debug, Serialize)]
#[skip_serializing_none]
pub struct GetQuery {
    pub since: Option<u64>,
    pub nb: Option<u32>,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self::get("/api/timeline", query, None)
    }
}

impl From<GetQuery> for GetRequest {
    fn from(query: GetQuery) -> Self {
        Self::new(query)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timeline {
    pub entries: Vec<TimelineEntry>,
    pub users: HashMap<String, TimelineUser>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineUser {
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    pub flair: Option<String>,
    pub patron: Option<bool>,
    #[serde(rename = "patronColor")]
    pub patron_color: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum TimelineEntry {
    Follow { data: FollowData, date: u64 },
    TeamJoin { data: TeamJoinData, date: u64 },
    TeamCreate { data: TeamCreateData, date: u64 },
    ForumPost { data: ForumPostData, date: u64 },
    BlogPost { data: BlogPostData, date: u64 },
    UblogPost { data: UblogPostData, date: u64 },
    TourJoin { data: TourJoinData, date: u64 },
    GameEnd { data: GameEndData, date: u64 },
    SimulCreate { data: SimulData, date: u64 },
    SimulJoin { data: SimulData, date: u64 },
    StudyLike { data: StudyLikeData, date: u64 },
    PlanStart { data: PlanStartData, date: u64 },
    PlanRenew { data: PlanRenewData, date: u64 },
    UblogPostLike { data: UblogPostLikeData, date: u64 },
    StreamStart { data: StreamStartData, date: u64 },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FollowData {
    pub u1: String,
    pub u2: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamJoinData {
    pub user_id: String,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamCreateData {
    pub user_id: String,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumPostData {
    pub user_id: String,
    pub topic_id: String,
    pub topic_name: String,
    pub post_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlogPostData {
    pub id: String,
    pub slug: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UblogPostData {
    pub user_id: String,
    pub id: String,
    pub slug: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TourJoinData {
    pub user_id: String,
    pub tour_id: String,
    pub tour_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameEndData {
    pub full_id: String,
    pub opponent: String,
    pub win: bool,
    pub perf: PerfType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulData {
    pub user_id: String,
    pub simul_id: String,
    pub simul_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyLikeData {
    pub user_id: String,
    pub study_id: String,
    pub study_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanStartData {
    pub user_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanRenewData {
    pub user_id: String,
    pub months: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UblogPostLikeData {
    pub user_id: String,
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamStartData {
    pub id: String,
    pub title: String,
}
