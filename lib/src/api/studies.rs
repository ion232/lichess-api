//! Create, import into, and export [studies](https://lichess.org/study) and
//! their chapters.
//!
//! Studies can be exported as PGN, either a single chapter, a whole study, or
//! every study belonging to a user; [`update_study_chapter_moves`] and
//! [`update_study_chapter_tags`] edit an existing chapter's move tree and PGN
//! tags in place. See [`model::studies`] for the request/response types.
//!
//! Reading a study you don't own only returns it if it's public. Pass a
//! bearer token to also see your own private and unlisted studies; writes
//! (creating, importing, deleting, or editing a chapter) always require one.
//!
//! [`update_study_chapter_moves`]: LichessApi::update_study_chapter_moves
//! [`update_study_chapter_tags`]: LichessApi::update_study_chapter_tags
//! [`model::studies`]: crate::model::studies

use futures::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::studies::create::{CreateStudyForm, CreateStudyResponse};
use crate::model::studies::import_pgn_into_study::{PostRequest, StudyImportPgnChapters};
use crate::model::studies::*;

impl LichessApi<reqwest::Client> {
    pub async fn create_study(&self, form: CreateStudyForm) -> Result<CreateStudyResponse> {
        self.get_single_model(create::PostRequest::new(form)).await
    }

    pub async fn import_pgn_into_study(
        &self,
        request: impl Into<PostRequest>,
    ) -> Result<StudyImportPgnChapters> {
        self.get_single_model(request.into()).await
    }

    pub async fn export_study_chapter_pgn(
        &self,
        study_id: &str,
        chapter_id: &str,
        query: export_chapter::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(export_chapter::GetRequest::new(study_id, chapter_id, query))
            .await
    }

    pub async fn export_study_pgn(
        &self,
        study_id: &str,
        query: export_study::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(export_study::GetRequest::new(study_id, query))
            .await
    }

    pub async fn get_study_metadata(
        &self,
        request: impl Into<study_metadata::HeadRequest>,
    ) -> Result<()> {
        self.get_empty(request.into()).await
    }

    pub async fn update_study_chapter_tags(
        &self,
        study_id: &str,
        chapter_id: &str,
        form: update_chapter_tags::UpdateChapterTagsForm,
    ) -> Result<()> {
        self.get_empty(update_chapter_tags::PostRequest::new(
            study_id, chapter_id, form,
        ))
        .await
    }

    pub async fn update_study_chapter_moves(
        &self,
        study_id: &str,
        chapter_id: &str,
        form: update_chapter_moves::UpdateChapterMovesForm,
    ) -> Result<()> {
        self.get_empty(update_chapter_moves::PostRequest::new(
            study_id, chapter_id, form,
        ))
        .await
    }

    pub async fn export_user_studies_pgn(
        &self,
        username: &str,
        query: export_user_studies::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(export_user_studies::GetRequest::new(username, query))
            .await
    }

    pub async fn list_user_studies(
        &self,
        request: impl Into<list_user_studies::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<StudyMetadata>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn delete_study_chapter(&self, study_id: &str, chapter_id: &str) -> Result<()> {
        self.get_empty(delete_chapter::DeleteRequest::new(study_id, chapter_id))
            .await
    }
}
