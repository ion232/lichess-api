use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde_with::skip_serializing_none]
pub struct GetQuery {
    game_id: String,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(game_id: &str) -> Self {
        Self {
            domain: crate::model::Domain::Explorer,
            path: format!("/masters/pgn/{}", game_id),
            query: Some(GetQuery {
                game_id: game_id.to_string(),
            }),
            ..Default::default()
        }
    }
}
