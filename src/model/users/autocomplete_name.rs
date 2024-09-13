use crate::model::Request;
use serde::Serialize;

#[serde_with::skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    term: String,
    friend: Option<bool>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(term: impl Into<String>, friend: Option<bool>) -> GetRequest {
        let term = term.into();
        Self::get("/api/autocomplete", GetQuery { term, friend }, None)
    }
}

impl<S: Into<String>> From<S> for GetRequest {
    fn from(s: S) -> Self {
        Self::new(s, None)
    }
}
