use crate::model::{PerfType, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(player_count: u8, perf_type: PerfType) -> Self {
        let url = format!("/api/player/top/{player_count}/{perf_type}");
        Self::get(url, None, None)
    }
}
