use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self {
            path: "/api/account/kid".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    pub v: bool,
}

pub type PostRequest = crate::model::Request<PostQuery>;

impl PostRequest {
    pub fn new(on: bool) -> Self {
        Self {
            method: http::Method::POST,
            path: "/api/account/kid".to_string(),
            query: Some(PostQuery { v: on }),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KidMode {
    pub kid: bool,
}
