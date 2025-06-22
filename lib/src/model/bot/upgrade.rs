use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new() -> Self {
        Self::post("/api/bot/account/upgrade", None, None, None)
    }
}

impl Default for PostRequest {
    fn default() -> Self {
        Self::new()
    }
}
