use crate::model::challenges::OpenChallenge;
use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, OpenChallenge>;

impl PostRequest {
    pub fn new(challenge: OpenChallenge) -> Self {
        Self {
            method: http::Method::POST,
            path: "/api/challenge/open".to_string(),
            body: Body::Form(challenge),
            ..Default::default()
        }
    }
}
