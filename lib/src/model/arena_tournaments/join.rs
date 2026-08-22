use crate::model::{Body, Request};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct JoinArenaTournamentForm {
    pub password: Option<String>,
    pub team: Option<String>,
    #[serde(rename = "pairMeAsap")]
    pub pair_me_asap: Option<bool>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, JoinArenaTournamentForm>;

impl PostRequest {
    pub fn new(id: &str, form: JoinArenaTournamentForm) -> Self {
        Self::post(
            format!("/api/tournament/{id}/join"),
            None,
            Body::Form(form),
            None,
        )
    }
}
