use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::openings::*;

impl LichessApi<reqwest::Client> {
    pub async fn openings_masters(
        &self,
        request: impl Into<masters::GetRequest>,
    ) -> Result<OpeningExplorerJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn openings_lichess(
        &self,
        request: impl Into<lichess::GetRequest>,
    ) -> Result<OpeningExplorerJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn openings_player(
        &self,
        request: impl Into<player::GetRequest>,
    ) -> Result<OpeningExplorerJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn openings_otb(
        &self,
        request: impl Into<otb::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(request.into()).await
    }
}
