use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::puzzles::*;

impl LichessApi<reqwest::Client> {
    pub async fn get_daily_puzzle(&self) -> Result<daily::Puzzle> {
        self.get_single_model(daily::GetRequest::new()).await
    }

    pub async fn get_puzzle(&self, request: impl Into<id::GetRequest>) -> Result<id::Puzzle> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_puzzle_activity(
        &self,
        request: impl Into<activity::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<activity::Round>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn get_puzzle_dashboard(
        &self,
        request: impl Into<dashboard::GetRequest>,
    ) -> Result<dashboard::Dashboard> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_puzzle_storm_dashboard(
        &self,
        request: impl Into<storm_dashboard::GetRequest>,
    ) -> Result<storm_dashboard::Dashboard> {
        self.get_single_model(request.into()).await
    }

    pub async fn make_puzzle_race(
        &self,
        request: impl Into<race::PostRequest>,
    ) -> Result<race::Race> {
        self.get_single_model(request.into()).await
    }
}
