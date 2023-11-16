// use crate::model::external_engine::ExternalEngineJson;;
use crate::model::{Domain, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self {
            domain: Domain::Engine,
            method: http::Method::GET,
            path: "/api/external-engine".to_string(),
            query: Default::default(),
            body: Default::default(),
        }
    }
}
