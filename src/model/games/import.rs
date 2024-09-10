use crate::model::Body;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

#[derive(Default, Clone, Debug, Serialize)]
pub struct Game {
    pgn: String,
}

pub type PostRequest = crate::model::Request<PostQuery, Game>;

impl PostRequest {
    pub fn new(pgn: String) -> Self {
        Self::post("/api/import", None, Body::Form(Game { pgn }), None)
    }
}

impl<S: Into<String>> From<S> for PostRequest {
    fn from(pgn: S) -> Self {
        Self::new(pgn.into())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportData {
    pub id: String,
    pub url: String,
}
