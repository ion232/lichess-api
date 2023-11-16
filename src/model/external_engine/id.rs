use serde::Serialize;

use crate::model::Domain;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(id: &str) -> Self {
        Self {
            domain: Domain::Engine,
            method: http::Method::GET,
            path: format!("/api/external-engine/{}", id),
            query: Default::default(),
            body: Default::default(),
        }
    }
}

pub type Engine = super::ExternalEngine;
