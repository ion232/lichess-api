use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub trophies: bool,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str, with_trophies: bool) -> Self {
        let query = GetQuery {
            trophies: with_trophies,
        };

        Self::get(format!("/api/user/{}", username), query, None)
    }
}
