use crate::model::{Body, Domain, Request};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, AnalysisRequest>;

impl PostRequest {
    pub fn new(id: &str, analysis_request: AnalysisRequest) -> Self {
        Self {
            domain: Domain::Engine,
            method: http::Method::POST,
            path: format!("/api/external-engine/{}/analyse", id),
            query: Default::default(),
            body: Body::Json(analysis_request),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisRequest {
    pub client_secret: String,
    pub work: ExternalEngineWork,
}

#[derive(Clone, Debug, Serialize)]
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
pub struct AnalysisResponse {
    pub time: u64,
    pub depth: u8,
    pub nodes: u64,
    pub pvs: Vec<PV>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PV {
    pub depth: u8,
    pub cp: i32,
    pub mate: i32,
    pub moves: Vec<String>,
}
