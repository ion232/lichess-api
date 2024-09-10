use crate::model::challenges::ChallengeJson;
use crate::model::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self::get("/api/challenge", None, None)
    }
}

impl Default for GetRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Challenges {
    pub r#in: Vec<ChallengeJson>,
    pub out: Vec<ChallengeJson>,
}
