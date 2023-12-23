use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub trophies: bool,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str, with_trophies: bool) -> Self {
        let path = format!("/api/user/{}", username);
        Self {
            path,
            query: Some(GetQuery {
                trophies: with_trophies,
            }),
            ..Default::default()
        }
    }
}
