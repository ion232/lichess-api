use crate::model::tv::ChannelName;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(channel: ChannelName) -> Self {
        Self::get(format!("/api/tv/{channel}/feed"), None, None)
    }
}

impl From<ChannelName> for GetRequest {
    fn from(channel: ChannelName) -> Self {
        Self::new(channel)
    }
}
