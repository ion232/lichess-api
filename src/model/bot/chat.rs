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
            method: http::Method::GET,
            path,
            query: Default::default(),
            body: Default::default(),
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
            query: Default::default(),
            body: Body::Form(message),
        }
    }
}

pub type ChatLine = crate::model::board::chat::ChatLine;
