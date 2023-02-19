use crate::model::Body;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

#[derive(Default, Clone, Debug, Serialize)]
pub struct Game {
    pgn: String
}

pub type PostRequest = crate::model::Request<PostQuery, Game>;

impl PostRequest {
    pub fn new(pgn: String) -> Self {
        Self {
            method: http::Method::POST,
            path: "/api/import".to_string(),
            query: Default::default(),
            body: Body::Form(Game {pgn}),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportData {
    pub id: String,
    pub url: String,
}
