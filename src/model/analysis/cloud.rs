use crate::model::VariantKey;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Default, Clone, Debug, Serialize)]
#[skip_serializing_none]
pub struct GetQuery {
    pub fen: String,
    #[serde(rename = "multiPv")]
    pub variation_count: Option<u32>,
    pub variant: Option<VariantKey>,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self {
            path: "/api/cloud-eval".to_string(),
            query: Some(query),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Evaluation {
    pub fen: String,
    #[serde(rename = "knodes")]
    pub k_nodes: u32,
    pub depth: u32,
    #[serde(rename = "pvs")]
    pub principal_variations: Vec<PrincipalVariation>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrincipalVariation {
    pub moves: String,
    #[serde(rename = "cp")]
    pub centipawns: i32,
}
