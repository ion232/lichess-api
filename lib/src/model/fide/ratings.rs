use crate::model::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(id: u32) -> Self {
        Self::get(format!("/api/fide/player/{id}/ratings"), None, None)
    }
}

impl From<u32> for GetRequest {
    fn from(id: u32) -> Self {
        Self::new(id)
    }
}

/// Each rating point encodes a year, month, and elo rating as a single number,
/// e.g. `2015081568` decodes to August 2015, elo 1568.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerRatings {
    pub standard: Vec<i64>,
    pub rapid: Vec<i64>,
    pub blitz: Vec<i64>,
}
