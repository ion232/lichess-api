use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct DeleteQuery;

pub type DeleteRequest = Request<DeleteQuery>;

impl DeleteRequest {
    pub fn new(study_id: &str, chapter_id: &str) -> Self {
        Self::delete(
            format!("/api/study/{study_id}/{chapter_id}"),
            None,
            None,
            None,
        )
    }
}
