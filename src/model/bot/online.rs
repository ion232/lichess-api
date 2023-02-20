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
            method: http::Method::GET,
            path: "/api/bot/online".to_string(),
            query: Some(GetQuery { nb: bot_count }),
            body: Default::default(),
        }
    }
}

pub type OnlineBot = crate::model::users::User;
