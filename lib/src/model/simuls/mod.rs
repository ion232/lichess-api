use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::LightUser;

use super::Variant;

pub mod current;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Simul {
    pub id: String,
    pub name: String,
    pub full_name: String,
    pub host: Host,
    pub is_created: bool,
    pub is_finished: bool,
    pub is_running: bool,
    pub estimated_start_at: u64,
    pub started_at: u64,
    pub finished_at: Option<u64>,
    #[serde(rename = "nbApplicants")]
    pub applicants: u32,
    #[serde(rename = "nbPairings")]
    pub pairings: u32,
    pub text: String,
    pub variants: Vec<Variant>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    #[serde(flatten)]
    pub user: LightUser,
    pub rating: u32,
    pub game_id: Option<String>,
    pub provisional: Option<bool>,
}
