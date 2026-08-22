use crate::model::{Body, Days, Request, VariantKey};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct CreateBulkPairingForm {
    pub players: String,
    #[serde(rename = "clock.limit")]
    pub clock_limit: Option<u32>,
    #[serde(rename = "clock.increment")]
    pub clock_increment: Option<u32>,
    pub days: Option<Days>,
    #[serde(rename = "pairAt")]
    pub pair_at: Option<i64>,
    #[serde(rename = "startClocksAt")]
    pub start_clocks_at: Option<i64>,
    pub rated: Option<bool>,
    pub variant: Option<VariantKey>,
    pub fen: Option<String>,
    pub message: Option<String>,
    pub rules: Option<String>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, CreateBulkPairingForm>;

impl PostRequest {
    pub fn new(form: CreateBulkPairingForm) -> Self {
        Self::post("/api/bulk-pairing", None, Body::Form(form), None)
    }
}
