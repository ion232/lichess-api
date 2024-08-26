use super::Base;
use crate::model::{Color, PerfType};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    #[serde(flatten)]
    pub base: Base,
    pub since: Option<u64>,
    pub until: Option<u64>,
    pub max: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated: Option<bool>,
    // Can be null.
    pub perf_type: Option<PerfType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Sort {
    DateAsc,
    DateDesc,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str, query: GetQuery) -> Self {
        let path = format!("/api/games/user/{}", username);
        Self {
            path,
            query: Some(query),
            ..Default::default()
        }
    }
}
