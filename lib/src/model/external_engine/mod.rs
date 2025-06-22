pub mod acquire_analysis;
pub mod analyse;
pub mod create;
pub mod delete;
pub mod id;
pub mod list;
pub mod update;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalEngine {
    pub id: String,
    pub name: String,
    pub client_secret: String,
    pub user_id: String,
    pub max_threads: u32,
    pub max_hash: u32,
    pub default_depth: u8,
    pub variants: Vec<String>,
    pub provider_data: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct CreateExternalEngine {
    pub name: String,
    pub max_threads: u32,
    pub max_hash: u32,
    pub default_depth: u8,
    pub variants: Option<Vec<String>>,
    pub provider_secret: String,
    pub provider_data: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct UpdateExternalEngine {
    pub name: String,
    pub max_threads: u32,
    pub max_hash: u32,
    pub default_depth: u8,
    pub variants: Option<Vec<String>>,
    pub provider_secret: String,
    pub provider_data: Option<String>,
}
