use crate::model::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(game_id: &str) -> Self {
        Self::get(format!("/api/game/{game_id}/chat"), None, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(s: S) -> Self {
        Self::new(s.as_ref())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatLine {
    pub text: String,
    pub user: String,
}
