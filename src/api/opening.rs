use crate::client::LichessApi;
use crate::error::Result;
use crate::model::opening::*;

impl LichessApi<reqwest::Client> {
    pub async fn openings_masters(&self, request: masters::GetRequest) -> Result<OpeningExplorerJson> {
        self.get_single_model(request).await
    }

    pub async fn openings_lichess(&self, request: lichess::GetRequest) -> Result<OpeningExplorerJson> {
        self.get_single_model(request).await
    }
}
