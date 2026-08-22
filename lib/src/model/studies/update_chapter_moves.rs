use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct UpdateChapterMovesForm {
    pub pgn: String,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, UpdateChapterMovesForm>;

impl PostRequest {
    pub fn new(study_id: &str, chapter_id: &str, form: UpdateChapterMovesForm) -> Self {
        Self::post(
            format!("/api/study/{study_id}/{chapter_id}/moves"),
            None,
            Body::Form(form),
            None,
        )
    }
}
