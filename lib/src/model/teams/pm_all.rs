use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

#[derive(Clone, Debug, Serialize)]
pub struct PmAllForm {
    pub message: String,
}

pub type PostRequest = Request<PostQuery, PmAllForm>;

impl PostRequest {
    pub fn new(team_id: &str, message: impl Into<String>) -> Self {
        let form = PmAllForm {
            message: message.into(),
        };

        Self::post(
            format!("/team/{team_id}/pm-all"),
            None,
            Body::Form(form),
            None,
        )
    }
}
