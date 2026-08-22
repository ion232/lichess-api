use crate::model::{Body, Domain, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, String>;

impl PostRequest {
    pub fn new(id: &str, analysis: String) -> Self {
        Self {
            domain: Domain::Engine,
            method: http::Method::POST,
            path: format!("/api/external-engine/work/{id}"),
            query: Default::default(),
            body: Body::PlainText(analysis),
        }
    }
}
