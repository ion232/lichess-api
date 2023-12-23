use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new() -> Self {
        Self {
            method: http::Method::POST,
            path: "/api/bot/account/upgrade".to_string(),
            ..Default::default()
        }
    }
}
