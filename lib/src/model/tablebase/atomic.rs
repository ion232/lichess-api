use crate::model::{Domain, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    fen: String,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(fen: impl Into<String>) -> Self {
        Self::get(
            "/atomic",
            GetQuery { fen: fen.into() },
            Some(Domain::Tablebase),
        )
    }
}

impl<S: Into<String>> From<S> for GetRequest {
    fn from(fen: S) -> Self {
        Self::new(fen)
    }
}
