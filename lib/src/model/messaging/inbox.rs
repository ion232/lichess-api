use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

#[derive(Default, Clone, Debug, Serialize)]
pub struct Message {
    text: String,
}

pub type PostRequest = Request<PostQuery, Message>;

impl PostRequest {
    pub fn new(username: &str, message: &str) -> Self {
        let message = Message {
            text: message.to_string(),
        };

        Self::post(
            format!("/inbox/{username}"),
            None,
            Body::Form(message),
            None,
        )
    }
}
