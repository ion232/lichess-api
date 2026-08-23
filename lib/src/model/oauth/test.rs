use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(tokens: Vec<String>) -> Self {
        let path = "/api/token/test".to_string();
        Self::post(path, None, Body::PlainText(tokens.join(",")), None)
    }
}
