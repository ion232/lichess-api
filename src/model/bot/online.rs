use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub nb: u32,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(bot_count: u32) -> Self {
        Self {
            path: "/api/bot/online".to_string(),
            query: Some(GetQuery { nb: bot_count }),
            ..Default::default()
        }
    }
}

pub type OnlineBot = crate::model::users::User;
