use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(id: &str) -> Self {
        Self {
            path: format!("/api/external-engine/{}", id),
            ..Default::default()
        }
    }
}

pub type Engine = super::ExternalEngine;
