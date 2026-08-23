//! Register and use an external engine: a chess engine running on the user's
//! own machine, made available for cloud analysis (e.g. from the Lichess
//! analysis board) via a provider/secret handshake.
//!
//! Listing, creating, fetching, updating, and deleting engine registrations
//! ([`list_external_engines`](LichessApi::list_external_engines),
//! [`create_external_engine`](LichessApi::create_external_engine),
//! [`get_external_engine`](LichessApi::get_external_engine),
//! [`update_external_engine`](LichessApi::update_external_engine),
//! [`delete_external_engine`](LichessApi::delete_external_engine)) require a
//! bearer token with the `engine:read` or `engine:write` scope and talk to
//! the regular Lichess host. Requesting and providing analysis
//! ([`analyse_with_external_engine`](LichessApi::analyse_with_external_engine),
//! [`acquire_analysis_request`](LichessApi::acquire_analysis_request),
//! [`submit_analysis`](LichessApi::submit_analysis)) instead use the engine's
//! own client/provider secrets for auth and are served from a separate host,
//! [`Domain::Engine`](crate::model::Domain::Engine).

use futures::stream::StreamExt;

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

    pub async fn submit_analysis(
        &self,
        request: impl Into<submit_analysis::PostRequest>,
    ) -> Result<()> {
        self.get_empty(request.into()).await
    }
}
