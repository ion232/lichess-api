use crate::model::tv::ChannelName;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde_with::skip_serializing_none]
pub struct GetQuery {
    #[serde(rename = "nb")]
    pub number_of_games: Option<u8>,
    pub moves: Option<bool>,
    #[serde(rename = "pgnInJson")]
    pub pgn_in_json: Option<bool>,
    pub tags: Option<bool>,
    pub clocks: Option<bool>,
    pub opening: Option<bool>,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(channel: ChannelName, query: Option<GetQuery>) -> Self {
        let path = format!("/api/tv/{channel}");

        Self {
            path,
            query,
            ..Default::default()
        }
    }
}
