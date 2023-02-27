use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Reason {
    Generic,
    Later,
    TooFast,
    TooSlow,
    TimeControl,
    Rated,
    Casual,
    Standard,
    Variant,
    NoBot,
    OnlyBot,
}

pub type PostRequest = Request<PostQuery, Reason>;

impl PostRequest {
    pub fn new(challenge_id: String, reason: Reason) -> Self {
        let path = format!("/api/challenge/{}/decline", challenge_id);
        Self {
            method: http::Method::POST,
            path,
            query: Default::default(),
            body: Body::Form(reason),
        }
    }
}
