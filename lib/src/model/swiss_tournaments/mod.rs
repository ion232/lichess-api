pub mod create;
pub mod games;
pub mod join;
pub mod results;
pub mod schedule_next_round;
pub mod show;
pub mod terminate;
pub mod trf;
pub mod update;
pub mod withdraw;

use crate::model::Title;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwissResult {
    pub rank: i32,
    pub points: f64,
    pub tie_break: i32,
    pub rating: i32,
    pub username: String,
    pub performance: i32,
    pub absent: Option<bool>,
    pub title: Option<Title>,
}
