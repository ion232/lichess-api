use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::external_engine::*;

impl LichessApi<reqwest::Client> {
    pub async fn list_external_engines(&self) -> Result<Vec<ExternalEngine>> {
        self.get_single_model(list::GetRequest::new()).await
    }

    pub async fn create_external_engine(
        &self,
        request: impl Into<create::PostRequest>,
    ) -> Result<ExternalEngine> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_external_engine(
        &self,
        request: impl Into<id::GetRequest>,
    ) -> Result<ExternalEngine> {
        self.get_single_model(request.into()).await
    }

    pub async fn update_external_engine(
        &self,
        request: impl Into<update::PutRequest>,
    ) -> Result<ExternalEngine> {
        self.get_single_model(request.into()).await
    }

    pub async fn delete_external_engine(
        &self,
        request: impl Into<delete::DeleteRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    /// This method currently returns a 503 error (Service Unavailable) from the Lichess API
    pub async fn analyse_with_external_engine(
        &self,
        request: impl Into<analyse::PostRequest>,
    ) -> Result<impl StreamExt<Item = Result<analyse::AnalysisResponse>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn acquire_analysis_request(
        &self,
        request: impl Into<acquire_analysis::PostRequest>,
    ) -> Result<Option<acquire_analysis::AcquireAnalysisResponse>> {
        let mut stream = self.get_streamed_models(request.into()).await?;
        // The response is a stream of 0 or 1 items, so we can just take the first item
        Ok((stream.next().await).transpose()?)
    }
}
