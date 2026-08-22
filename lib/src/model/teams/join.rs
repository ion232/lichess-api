use crate::model::{Body, Request};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct JoinForm {
    pub message: Option<String>,
    pub password: Option<String>,
}

pub type PostRequest = Request<PostQuery, JoinForm>;

impl PostRequest {
    pub fn new(team_id: &str, form: JoinForm) -> Self {
        Self::post(
            format!("/team/{team_id}/join"),
            None,
            Body::Form(form),
            None,
        )
    }
}

impl<S: AsRef<str>> From<S> for PostRequest {
    fn from(team_id: S) -> Self {
        Self::new(team_id.as_ref(), JoinForm::default())
    }
}
