use serde::{Deserialize, Serialize};

use crate::model::{Domain, Request};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde_with::skip_serializing_none]
pub struct GetQuery {
    game_id: String,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(game_id: impl Into<String>) -> Self {
        let game_id = game_id.into();
        Self::get(
            format!("/masters/pgn/{game_id}"),
            GetQuery { game_id },
            Domain::Explorer,
        )
    }
}

impl<S: Into<String>> From<S> for GetRequest {
    fn from(s: S) -> Self {
        Self::new(s)
    }
}
