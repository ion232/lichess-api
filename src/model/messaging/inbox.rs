use serde::Serialize;
use crate::model::{Request, Body};

#[derive(Default, Clone, Debug, Serialize)]
pub struct Query;

#[derive(Default, Clone, Debug, Serialize)]
pub struct Message {
    text: String
}

pub type PostRequest = Request<Query, Message>;

impl PostRequest {
    pub fn new(username: &str, message: &str) -> Self {
        let message = Message{text: message.to_string()};
        Self {
            method: http::Method::POST,
            path: format!("/inbox/{}", username).to_string(),
            query: Default::default(),
            body: Body::Form(message)
        }
    }
}
