use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::opening::*;

impl LichessApi<reqwest::Client> {
    pub async fn openings_masters(
        &self,
        request: masters::GetRequest,
    ) -> Result<OpeningExplorerJson> {
        self.get_single_model(request).await
    }

    pub async fn openings_lichess(
        &self,
        request: lichess::GetRequest,
    ) -> Result<OpeningExplorerJson> {
        self.get_single_model(request).await
    }

    pub async fn openings_otb(
        &self,
        request: otb::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(request).await
    }
}
