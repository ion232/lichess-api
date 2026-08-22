pub mod create;
pub mod delete_chapter;
pub mod export_chapter;
pub mod export_study;
pub mod export_user_studies;
pub mod import_pgn_into_study;
pub mod list_user_studies;
pub mod study_metadata;
pub mod update_chapter_moves;
pub mod update_chapter_tags;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct PgnExportQuery {
    pub clocks: Option<bool>,
    pub comments: Option<bool>,
    pub variations: Option<bool>,
    pub orientation: Option<bool>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyMetadata {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}
