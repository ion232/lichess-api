use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(
        broadcast_tournament_slug: &str,
        broadcast_round_slug: &str,
        broadcast_round_id: &str,
    ) -> Self {
        Self::get(
            format!(
                "/api/broadcast/{broadcast_tournament_slug}/{broadcast_round_slug}/{broadcast_round_id}"
            ),
            None,
            None,
        )
    }
}
