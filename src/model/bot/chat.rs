use crate::model::board::chat::ChatMessage;
use crate::model::{Body, Request, Room};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(game_id: &str) -> Self {
        let path = format!("/api/bot/game/{}/chat", game_id);
        Self {
            path,
            ..Default::default()
        }
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
        let path = format!("/api/bot/game/{}/chat", game_id);
        Self {
            method: http::Method::POST,
            path,
            body: Body::Form(message),
            ..Default::default()
        }
    }
}

pub type ChatLine = crate::model::board::chat::ChatLine;
