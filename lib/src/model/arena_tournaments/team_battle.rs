use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

#[derive(Clone, Debug, Serialize)]
pub struct TeamBattleForm {
    pub teams: String,
    #[serde(rename = "nbLeaders")]
    pub nb_leaders: u32,
}

pub type PostRequest = Request<PostQuery, TeamBattleForm>;

impl PostRequest {
    pub fn new(id: &str, form: TeamBattleForm) -> Self {
        Self::post(
            format!("/api/tournament/team-battle/{id}"),
            None,
            Body::Form(form),
            None,
        )
    }
}
