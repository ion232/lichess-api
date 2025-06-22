use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Title, Variant};

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
    pub id: String,
    pub name: String,
    pub rating: u32,
    pub title: Option<Title>,
    pub game_id: Option<String>,
    pub online: Option<bool>,
    pub provisional: Option<bool>,
}
