use crate::model::Request;
use crate::model::broadcasts::PgnStreamQuery;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    #[serde(flatten)]
    pub options: PgnStreamQuery,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(broadcast_round_id: &str, query: GetQuery) -> Self {
        Self::get(
            format!("/api/broadcast/round/{broadcast_round_id}.pgn"),
            query,
            None,
        )
    }
}
