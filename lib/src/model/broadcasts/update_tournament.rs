use crate::model::broadcasts::create_tournament::CreateBroadcastTournamentForm;
use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, CreateBroadcastTournamentForm>;

impl PostRequest {
    pub fn new(broadcast_tournament_id: &str, form: CreateBroadcastTournamentForm) -> Self {
        Self::post(
            format!("/broadcast/{broadcast_tournament_id}/edit"),
            None,
            Body::Form(form),
            None,
        )
    }
}
