use crate::model::broadcasts::create_round::BroadcastRoundForm;
use crate::model::{Body, Request};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    pub patch: Option<bool>,
}

pub type PostRequest = Request<PostQuery, BroadcastRoundForm>;

impl PostRequest {
    pub fn new(broadcast_round_id: &str, query: PostQuery, form: BroadcastRoundForm) -> Self {
        Self::post(
            format!("/broadcast/round/{broadcast_round_id}/edit"),
            query,
            Body::Form(form),
            None,
        )
    }
}
