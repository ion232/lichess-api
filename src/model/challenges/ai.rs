use crate::model::challenges::AIChallenge;
use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, AIChallenge>;

impl PostRequest {
    pub fn new(challenge: AIChallenge) -> Self {
        Self {
            method: http::Method::POST,
            path: "/api/challenge/ai".to_string(),
            query: Default::default(),
            body: Body::Form(challenge),
        }
    }
}
