use crate::client::LichessApi;
use crate::error::Result;
use crate::model::studies::import_pgn_into_study::{PostRequest, StudyImportPgnChapters};

impl LichessApi<reqwest::Client> {
    pub async fn import_pgn_into_study(
        &self,
        request: impl Into<PostRequest>,
    ) -> Result<StudyImportPgnChapters> {
        self.get_single_model(request.into()).await
    }
}
