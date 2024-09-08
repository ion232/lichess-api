use crate::model::challenges::OpenChallenge;
use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, OpenChallenge>;

impl PostRequest {
    pub fn new(challenge: OpenChallenge) -> Self {
        Self::post("/api/challenge/open", None, Body::Form(challenge), None)
    }
}

impl From<OpenChallenge> for PostRequest {
    fn from(challenge: OpenChallenge) -> Self {
        Self::new(challenge)
    }
}
