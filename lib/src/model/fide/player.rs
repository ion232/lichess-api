use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(id: u32) -> Self {
        Self::get(format!("/api/fide/player/{id}"), None, None)
    }
}

impl From<u32> for GetRequest {
    fn from(id: u32) -> Self {
        Self::new(id)
    }
}
