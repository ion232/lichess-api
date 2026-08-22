use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery>;

impl PostRequest {
    pub fn new(broadcast_round_id: &str) -> Self {
        Self::post(
            format!("/broadcast/round/{broadcast_round_id}/reset"),
            None,
            None,
            None,
        )
    }
}

impl<S: AsRef<str>> From<S> for PostRequest {
    fn from(broadcast_round_id: S) -> Self {
        Self::new(broadcast_round_id.as_ref())
    }
}
