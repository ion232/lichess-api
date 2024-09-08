use crate::model::challenges::AIChallenge;
use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, AIChallenge>;

impl PostRequest {
    pub fn new(challenge: AIChallenge) -> Self {
        Self::post("/api/challenge/ai", None, Body::Form(challenge), None)
    }
}

impl From<AIChallenge> for PostRequest {
    fn from(challenge: AIChallenge) -> Self {
        Self::new(challenge)
    }
}
