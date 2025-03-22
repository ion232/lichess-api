use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::fide::*;

impl LichessApi<reqwest::Client> {
    pub async fn search_fide_player(
        &self,
        request: impl Into<search::GetRequest>,
    ) -> Result<Vec<Player>> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_fide_player(
        &self,
        request: impl Into<player::GetRequest>,
    ) -> Result<Player> {
        self.get_single_model(request.into()).await
    }
}
