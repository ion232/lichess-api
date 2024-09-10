use crate::model::board::chat::ChatMessage;
use crate::model::{Body, Request, Room};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(game_id: &str) -> Self {
        Self::get(format!("/api/bot/game/{game_id}/chat"), None, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(s: S) -> Self {
        Self::new(s.as_ref())
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, ChatMessage>;

impl PostRequest {
    pub fn new(game_id: &str, room: Room, message: &str) -> Self {
        let message = ChatMessage {
            room,
            text: message.to_string(),
        };
        let path = format!("/api/bot/game/{game_id}/chat");
        Self::post(path, None, Body::Form(message), None)
    }
}

pub type ChatLine = crate::model::board::chat::ChatLine;
