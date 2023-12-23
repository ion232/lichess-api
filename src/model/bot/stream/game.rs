use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(game_id: &str) -> Self {
        let path = format!("/api/bot/game/stream/{}", game_id);
        Self {
            path,
            ..Default::default()
        }
    }
}

pub type Event = crate::model::board::stream::game::Event;
