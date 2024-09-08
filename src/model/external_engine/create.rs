use crate::model::external_engine::CreateExternalEngine;
use crate::model::{Body, Request};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, CreateExternalEngine>;

impl PostRequest {
    pub fn new(engine: CreateExternalEngine) -> Self {
        Self::post("/api/external-engine", None, Body::Json(engine), None)
    }
}

impl From<CreateExternalEngine> for PostRequest {
    fn from(engine: CreateExternalEngine) -> Self {
        Self::new(engine)
    }
}

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
