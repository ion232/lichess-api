use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct HeadQuery;

pub type HeadRequest = Request<HeadQuery>;

impl HeadRequest {
    pub fn new(study_id: &str) -> Self {
        Self::head(format!("/api/study/{study_id}.pgn"), None, None)
    }
}

impl<S: AsRef<str>> From<S> for HeadRequest {
    fn from(study_id: S) -> Self {
        Self::new(study_id.as_ref())
    }
}
