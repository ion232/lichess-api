use crate::model::Request;
use crate::model::studies::PgnExportQuery;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    #[serde(flatten)]
    pub options: PgnExportQuery,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(study_id: &str, chapter_id: &str, query: GetQuery) -> Self {
        Self::get(
            format!("/api/study/{study_id}/{chapter_id}.pgn"),
            query,
            None,
        )
    }
}
