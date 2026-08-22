use crate::model::broadcasts::{BroadcastCustomPointsPerColor, BroadcastCustomScoring};
use crate::model::{Body, Request};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct BroadcastRoundForm {
    pub name: String,
    #[serde(rename = "syncUrl")]
    pub sync_url: Option<String>,
    #[serde(rename = "syncUrls")]
    pub sync_urls: Option<String>,
    #[serde(rename = "syncIds")]
    pub sync_ids: Option<String>,
    #[serde(rename = "syncUsers")]
    pub sync_users: Option<String>,
    #[serde(rename = "onlyRound")]
    pub only_round: Option<i32>,
    pub slices: Option<String>,
    #[serde(rename = "syncSource")]
    pub sync_source: Option<String>,
    #[serde(rename = "startsAt")]
    pub starts_at: Option<i64>,
    #[serde(rename = "startsAfterPrevious")]
    pub starts_after_previous: Option<bool>,
    pub delay: Option<i32>,
    pub status: Option<String>,
    pub rated: Option<bool>,
    #[serde(rename = "customScoring")]
    pub custom_scoring: Option<BroadcastCustomScoring>,
    #[serde(rename = "teamCustomScoring")]
    pub team_custom_scoring: Option<BroadcastCustomPointsPerColor>,
    pub period: Option<i32>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, BroadcastRoundForm>;

impl PostRequest {
    pub fn new(broadcast_tournament_id: &str, form: BroadcastRoundForm) -> Self {
        Self::post(
            format!("/broadcast/{broadcast_tournament_id}/new"),
            None,
            Body::Form(form),
            None,
        )
    }
}
