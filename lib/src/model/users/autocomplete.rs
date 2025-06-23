use crate::model::{LightUser, Request};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Autocompletions {
    pub result: Vec<LightUser>,
}

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

        Self::get("/api/player/autocomplete", query, None)
    }
}

impl<S: Into<String>> From<S> for GetRequest {
    fn from(s: S) -> Self {
        Self::new(s, None)
    }
}
