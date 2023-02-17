use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self {
            method: http::Method::GET,
            path: "/api/account/profile".to_string(),
            query: Default::default(),
            body: Default::default(),
        }
    }
}

pub type Profile = crate::model::users::UserExtended;
