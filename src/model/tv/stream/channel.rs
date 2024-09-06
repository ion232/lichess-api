use crate::model::tv::ChannelName;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(channel: ChannelName) -> Self {
        let path = format!("/api/tv/{channel}/feed");

        Self {
            path,
            ..Default::default()
        }
    }
}
