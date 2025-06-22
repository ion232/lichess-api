use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Default, Clone, Debug, Serialize)]
#[skip_serializing_none]
pub struct GetQuery {
    #[serde(rename = "q")]
    pub query: String,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(query: GetQuery) -> Self {
        Self::get("/api/fide/player", query, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(query: S) -> Self {
        GetQuery {
            query: query.as_ref().to_string(),
        }
        .into()
    }
}

impl From<GetQuery> for GetRequest {
    fn from(query: GetQuery) -> Self {
        Self::new(query)
    }
}
