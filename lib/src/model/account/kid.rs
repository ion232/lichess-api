use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self::get("/api/account/kid", None, None)
    }
}

impl Default for GetRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    pub v: bool,
}

pub type PostRequest = crate::model::Request<PostQuery>;

impl PostRequest {
    pub fn new(on: bool) -> Self {
        Self::get("/api/account/kid", PostQuery { v: on }, None)
    }
}

impl From<bool> for PostRequest {
    fn from(on: bool) -> Self {
        Self::new(on)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KidMode {
    pub kid: bool,
}
