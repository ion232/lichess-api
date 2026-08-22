use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(broadcast_tournament_id: &str, player_id: &str) -> Self {
        Self::get(
            format!("/broadcast/{broadcast_tournament_id}/players/{player_id}"),
            None,
            None,
        )
    }
}
