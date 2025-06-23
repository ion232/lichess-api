use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self::get("/api/streamer/live", None, None)
    }
}

impl Default for GetRequest {
    fn default() -> Self {
        Self::new()
    }
}
