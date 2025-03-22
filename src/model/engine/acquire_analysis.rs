use crate::model::{Body, Domain, Request};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, AcquireAnalysis>;

impl PostRequest {
    pub fn new(acquire_analysis: AcquireAnalysis) -> Self {
        Self {
            domain: Domain::Engine,
            method: http::Method::POST,
            path: format!("/api/external-engine/work"),
            query: Default::default(),
            body: Body::Json(acquire_analysis),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcquireAnalysis {
    pub provider_secret: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcquireAnalysisResponse {
    pub id: String,
    pub work: ExternalEngineWork,
    pub engine: Engine,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalEngineWork {
    pub session_id: String,
    pub threads: u32,
    pub hash: u32,
    pub infinite: bool,
    pub multi_pv: u8,
    pub variant: String,
    pub initial_fen: String,
    pub moves: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Engine {
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
