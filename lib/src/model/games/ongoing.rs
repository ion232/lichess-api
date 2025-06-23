use crate::model::board::stream::events::GameEventInfo;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub max_games: u8,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(max_games: u8) -> Self {
        Self::get("/api/account/playing", GetQuery { max_games }, None)
    }
}

impl From<u8> for GetRequest {
    fn from(max_games: u8) -> Self {
        Self::new(max_games)
    }
}

impl Default for GetRequest {
    fn default() -> Self {
        Self::new(9)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    pub now_playing: Vec<GameEventInfo>,
}
