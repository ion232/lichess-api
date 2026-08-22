use crate::model::{Body, Request};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct ScheduleNextRoundForm {
    pub date: i64,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, ScheduleNextRoundForm>;

impl PostRequest {
    pub fn new(id: &str, form: ScheduleNextRoundForm) -> Self {
        Self::post(
            format!("/api/swiss/{id}/schedule-next-round"),
            None,
            Body::Form(form),
            None,
        )
    }
}
