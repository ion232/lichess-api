use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub nb: u32,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(bot_count: u32) -> Self {
        Self::get("/api/bot/online", GetQuery { nb: bot_count }, None)
    }
}

impl From<u32> for GetRequest {
    fn from(bot_count: u32) -> Self {
        Self::new(bot_count)
    }
}

pub type OnlineBot = crate::model::users::User;
