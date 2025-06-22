use serde::{Deserialize, Serialize};

use super::Simul;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self::get("/api/simul", None, None)
    }
}

impl Default for GetRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simuls {
    pub pending: Vec<Simul>,
    pub created: Vec<Simul>,
    pub started: Vec<Simul>,
    pub finished: Vec<Simul>,
}
