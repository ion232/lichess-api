use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, AdminChallengeTokens>;

impl PostRequest {
    pub fn new(tokens: AdminChallengeTokens) -> Self {
        let path = "/api/token/admin-challenge".to_string();
        Self::post(path, None, Body::Form(tokens), None)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AdminChallengeTokens {
    /// Usernames separated with commas
    pub users: String,
    /// User visible description of the token
    pub description: String,
}

impl From<AdminChallengeTokens> for PostRequest {
    fn from(tokens: AdminChallengeTokens) -> Self {
        Self::new(tokens)
    }
}
