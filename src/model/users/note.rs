use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str) -> Self {
        Self::get(format!("/api/user/{username}/note"), None, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(username: S) -> Self {
        Self::new(username.as_ref())
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

#[derive(Default, Clone, Debug, Serialize)]
pub struct Message {
    text: String,
}

pub type PostRequest = Request<PostQuery, Message>;

impl PostRequest {
    pub fn new(username: &str, message: impl Into<String>) -> Self {
        let path = format!("/api/user/{username}/note");
        let message = Message {
            text: message.into(),
        };

        Self::post(path, None, Body::Form(message), None)
    }
}
