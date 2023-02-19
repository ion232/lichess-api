use crate::model::board::stream_events::GameEventInfo;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub max_games: u8,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(max_games: u8) -> Self {
        Self {
            method: http::Method::GET,
            path: "/api/account/playing".to_string(),
            query: Some(GetQuery { max_games }),
            body: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    pub now_playing: Vec<GameEventInfo>,
}
