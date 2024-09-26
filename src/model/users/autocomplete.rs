use crate::model::Request;
use serde::Serialize;

#[serde_with::skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    term: String,
    object: bool,
    friend: Option<bool>,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(term: impl Into<String>, friend: Option<bool>) -> Self {
        let query = GetQuery {
            term: term.into(),
            object: true,
            friend,
        };

        Self::get("/api/autocomplete", query, None)
    }
}

impl<S: Into<String>> From<S> for GetRequest {
    fn from(s: S) -> Self {
        Self::new(s, None)
    }
}
