use crate::model::{Body, Request};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Unlisted,
    Private,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StudyUserSelection {
    Nobody,
    Owner,
    Contributor,
    Member,
    Everyone,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct CreateStudyForm {
    pub name: String,
    pub visibility: Visibility,
    pub flair: Option<String>,
    pub computer: StudyUserSelection,
    pub explorer: StudyUserSelection,
    pub cloneable: StudyUserSelection,
    pub shareable: StudyUserSelection,
    pub chat: StudyUserSelection,
    pub sticky: Option<bool>,
    pub description: Option<bool>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct CreateStudyResponse {
    pub id: String,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, CreateStudyForm>;

impl PostRequest {
    pub fn new(form: CreateStudyForm) -> Self {
        Self::post("/api/study", None, Body::Form(form), None)
    }
}
