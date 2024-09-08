use crate::model::Title;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQuery {
    pub ids: String,
    pub with_game_ids: bool,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(user_ids: Vec<String>, with_game_ids: bool) -> Self {
        Self::get(
            "/api/users/status",
            Some(GetQuery {
                ids: user_ids.join(","),
                with_game_ids,
            }),
            None,
        )
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    pub online: Option<bool>,
    pub playing: Option<bool>,
    pub streaming: Option<bool>,
    pub patron: Option<bool>,
}
