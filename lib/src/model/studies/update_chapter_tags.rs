use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct UpdateChapterTagsForm {
    pub pgn: String,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, UpdateChapterTagsForm>;

impl PostRequest {
    pub fn new(study_id: &str, chapter_id: &str, form: UpdateChapterTagsForm) -> Self {
        Self::post(
            format!("/api/study/{study_id}/{chapter_id}/tags"),
            None,
            Body::Form(form),
            None,
        )
    }
}
