use crate::model::{Body, Request, Room};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(game_id: &str) -> Self {
        let path = format!("/api/board/game/{}/chat", game_id);
        Self {
            method: http::Method::GET,
            path,
            query: Default::default(),
            body: Default::default(),
        }
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

#[derive(Clone, Debug, Serialize)]
pub struct ChatMessage {
    pub room: Room,
    pub text: String,
}

pub type PostRequest = Request<PostQuery, ChatMessage>;

impl PostRequest {
    pub fn new(game_id: &str, room: Room, message: &str) -> Self {
        let message = ChatMessage {
            room,
            text: message.to_string(),
        };
        let path = format!("/api/board/game/{}/chat", game_id);
        Self {
            method: http::Method::POST,
            path,
            query: Default::default(),
            body: Body::Form(message),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatLine {
    pub text: String,
    pub user: String,
}
