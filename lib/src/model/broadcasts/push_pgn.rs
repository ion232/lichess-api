use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, String>;

impl PostRequest {
    pub fn new(broadcast_round_id: &str, pgn: String) -> Self {
        Self::post(
            format!("/broadcast/round/{broadcast_round_id}/push"),
            None,
            Body::PlainText(pgn),
            None,
        )
    }
}
