use crate::model::{Body, Request};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct JoinSwissTournamentForm {
    pub password: Option<String>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, JoinSwissTournamentForm>;

impl PostRequest {
    pub fn new(id: &str, form: JoinSwissTournamentForm) -> Self {
        Self::post(
            format!("/api/swiss/{id}/join"),
            None,
            Body::Form(form),
            None,
        )
    }
}
