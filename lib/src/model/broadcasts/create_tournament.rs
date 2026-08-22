use crate::model::broadcasts::BroadcastTiebreakExtendedCode;
use crate::model::{Body, Request};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct CreateBroadcastTournamentForm {
    pub name: String,
    #[serde(rename = "info.format")]
    pub info_format: Option<String>,
    #[serde(rename = "info.tc")]
    pub info_tc: Option<String>,
    #[serde(rename = "info.fideTC")]
    pub info_fide_tc: Option<String>,
    #[serde(rename = "info.timeZone")]
    pub info_time_zone: Option<String>,
    #[serde(rename = "info.location")]
    pub info_location: Option<String>,
    #[serde(rename = "info.players")]
    pub info_players: Option<String>,
    #[serde(rename = "info.website")]
    pub info_website: Option<String>,
    #[serde(rename = "info.standings")]
    pub info_standings: Option<String>,
    #[serde(rename = "info.regulations")]
    pub info_regulations: Option<String>,
    pub markdown: Option<String>,
    #[serde(rename = "showScores")]
    pub show_scores: Option<bool>,
    #[serde(rename = "showRatingDiffs")]
    pub show_rating_diffs: Option<bool>,
    #[serde(rename = "teamTable")]
    pub team_table: Option<bool>,
    pub visibility: Option<String>,
    pub players: Option<String>,
    pub teams: Option<String>,
    pub tier: Option<i32>,
    pub tiebreaks: Option<Vec<BroadcastTiebreakExtendedCode>>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, CreateBroadcastTournamentForm>;

impl PostRequest {
    pub fn new(form: CreateBroadcastTournamentForm) -> Self {
        Self::post("/broadcast/new", None, Body::Form(form), None)
    }
}
